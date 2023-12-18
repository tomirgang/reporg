-- Your SQL goes here
INSERT INTO state (
    id,
    name,
    description,
    final
) VALUES
(0, 'neu', 'ungeprüfte Geräteanmeldung', 0),
(1, 'in Rückfrage', 'Für die Reparatur werden weitere Informationen benötigt.', 0),
(2, 'akzeptiert', 'geprüfte Geräteanmeldung', 0),
(3, 'abgelehnt', 'Gerät kann nicht im Rahmen eines Repair-Cafés repariert werden', 1),
(4, 'repariert', 'Geräte wurde erfolgreich repariert', 1),
(5, 'teil-repariert', 'Die Reparatur wurde begonnen und kann an einem zusätzlichen Termin fortgesetzt werden.', 0),
(6, 'defekt', 'Das Gerät konnte nicht repariert werden.', 1)