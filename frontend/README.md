# Frontend E2E Tests

## Vorbereitung

Diese Tests brauchen den Chromedriver.
Auf Debian kann der Chromedriver mit `sudo apt install chromium-driver` installiert werden.

## Tests ausführen

Zum Ausführen der Test folgende Schritte beachten:

1) Backend starten: In `backend`: `cargo run`
2) App starten: In `frontend`: `trunk serve`
3) Chromedriver starten: `chromedriver`
4) Tests ausführen: In `frontend` `cargo test`
