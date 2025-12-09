# Secure File Sharing (Rust + React + Hybrid Encryption)

Aplikacja do bezpiecznej, szyfrowanej wymiany plików (End-to-End Encryption) zrealizowana w ramach projektu
inżynierskiego. Projekt wykorzystuje nowoczesne technologie gwarantujące wysoką wydajność, bezpieczeństwo typów i
skalowalność.

## Technologie

W projekcie świadomie zastąpiono sugerowany stack (Python/Flask) nowocześniejszymi odpowiednikami:

* **Backend:** **Rust** + **Axum** (zamiast Flask).
    * *Dlaczego?* Rust gwarantuje bezpieczeństwo pamięci, wydajność (brak Garbage Collectora) i bezpieczną
      współbieżność, co jest kluczowe przy operacjach kryptograficznych i obsłudze plików.
* **Baza Danych:** **SurrealDB (Embedded RocksDB)** (zamiast SQLite).
    * *Dlaczego?* Nowoczesna baza dokumentowa, idealna do przechowywania metadanych JSON i list odbiorców. Działa w
      trybie wbudowanym (brak potrzeby instalacji serwera).
* **Frontend:** **React** + **TanStack Query** + **TypeScript** (zamiast HTML/Jinja2).
    * *Dlaczego?* Pozwala na wykonywanie operacji kryptograficznych (Web Crypto API) po stronie klienta (Client-Side
      Encryption) bez blokowania interfejsu.

## Kryptografia (Hybrid Encryption)

Aplikacja realizuje model **Szyfrowania Kopertowego (Envelope Encryption)**, co pozwala na bezpieczne przesyłanie
jednego pliku do wielu odbiorców.

### Jak to działa? (Algorytm)

1. **Generowanie Tożsamości (RSA-4096):**
    * Użytkownik generuje w przeglądarce parę kluczy RSA (4096-bit).
    * Klucz prywatny jest zapisywany w `localStorage` (nigdy nie opuszcza urządzenia).
    * Klucz publiczny jest wysyłany na serwer.

2. **Wysyłanie Pliku (Encryption):**
    * Nadawca generuje jednorazowy, symetryczny klucz sesji **AES-256-GCM**.
    * Plik jest szyfrowany kluczem AES (zapewnia to wydajność dla dużych plików).
    * Klucz AES jest szyfrowany asymetrycznie (**RSA-OAEP**) osobno dla każdego odbiorcy.
    * Na serwer trafia: Zaszyfrowany plik `.bin` + Metadane (lista zaszyfrowanych kluczy AES dla odbiorców).

3. **Odbieranie Pliku (Decryption):**
    * Odbiorca pobiera zaszyfrowany plik i metadane.
    * Odbiorca używa swojego Klucza Prywatnego RSA, aby odszyfrować klucz AES z metadanych.
    * Odzyskanym kluczem AES odszyfrowuje plik w pamięci przeglądarki.

### Spełnienie Wymagań (Checklist)

| Wymóg                               | Status | Implementacja                                          |
|:------------------------------------|:------:|:-------------------------------------------------------|
| **Hybrydowe szyfrowanie (RSA+AES)** |   ✅    | Web Crypto API (Frontend)                              |
| **Generowanie kluczy RSA**          |   ✅    | RSA-OAEP 4096-bit                                      |
| **Zapis klucza publicznego (PEM)**  |   ✅    | Endpoint `POST /users`, baza SurrealDB                 |
| **Obsługa plików > 50MB**           |   ✅    | Limit 100MB (skonfigurowalny w `axum`), strumieniowanie |
| **Backend API**                     |   ✅    | Rust / Axum (REST API)                                 |
| **Baza Danych**                     |   ✅    | SurrealDB (przechowuje klucze i metadane)              |
| **Weryfikacja integralności**       |   ✅    | Zapewniona przez AES-GCM (Authenticated Encryption)    |
| **Frontend UI**                     |   ✅    | React (Upload form, Lista plików, Deszyfrowanie)       |
| **Automatyczne usuwanie (24h)**     |   ✅    | Background Cleaner w Rust (`tokio::spawn`)             |
| **Historia transakcji**             |   ✅    | Widok `Secure Download` (lista plików usera)           |

## Instalacja i Uruchomienie

### Backend (Rust)

```bash
cd backend
# Baza danych zostanie utworzona automatycznie w folderze projektu
cargo run
```

Serwer nasłuchuje na porcie `3000`.

### Frontend (React)

```bash
cd frontend
npm install
npm run dev
```

Aplikacja dostępna pod `http://localhost:5173`.

## ⚠Uwagi Bezpieczeństwa (Proof of Concept)

* System tożsamości oparty jest na `localStorage` i UUID. W wersji produkcyjnej należałoby dodać autentykację (np.
  OIDC).
* Klucze prywatne są przechowywane w pamięci przeglądarki (exportable), co jest akceptowalne dla POC, ale wymagałoby
  zabezpieczenia hasłem w produkcji.
