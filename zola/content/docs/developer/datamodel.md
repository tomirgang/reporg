+++
title = "Datenmodel"
description = "Das RepOrg Datenmodel."
date = 2021-05-01T08:20:00+00:00
updated = 2021-05-01T08:20:00+00:00
draft = false
weight = 20
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = "Ein Überblick über das RepOrg Datenmodel."
toc = true
top = false
+++

RepOrg nimmt an dass alle verwalteten Geräteanmeldungen vor dem Repair-Cafe, z.B. über ein Web-Formular, erfolgen. Diese Geräteanmeldungen werden anschließend von den Organisatoren überprüft und Reparateuren zugeordnet.

RepOrg modelliert die folgenden Objekte:

- cafe: Die Repair-Cafe Veranstaltungen.
- device: Die defekten Geräte die für ein Repair-Cafe angemeldet wurden.
- guest: Die Besitzer der defekten Geräte, die ein Repair-Cafe besuchen.
- supporter: Die Helfer die am Repair-Cafe teilnehmen um Gästen bei der Reparatur ihrer Geräte zu helfen.

Zusätzlich gibt es die folgenden Hilfsobjekte:

- user: Die Benutzer regeln die Rechte im Web-Interface.
- meeting: Die Termine ordnen ein Gerät einem Repair-Cafe und einem Helfer zu.
- message: Die Kommunikation zu einem Gerät.
- state: Der Status einer Geräteanmeldung.

Wir versuchen die Reparaturen möglichst erfolgreich zu gestalten, dazu ist es notwendig sicherzustellen dass für jedes Gerät ein geeigneter Helfer zur Verfügung steht, und dass eine Reparatur in gegebenen Zeitrahmen erfolgversprechend ist. Um dies zue gewährleiten erfolgt für jedes Gerät eine Eingangsüberprüfung. Ein Gerät kann folgenden Status haben:

- new: Neue, ungeprüfte Geräteanmeldung.
- questioned: In Rückfrage. Falls nicht genügend Informationen vorliegen, kontaktieren die Organisatoren den Gerätebesitzer um die notwendigen Informationen einzuholen.
- waiting: Gerät wurde angenommen und kann für ein Repair-Cafe geplant werden.
- rejected: Das Gerät wurde abgelehnt, da eine erfolgreiche Reparatur unwahrscheinlich ist.
- repaired: Das Gerät wurde repariert.
- follow_up: Das Gerät wurde teilweise Repariert, und ein zweiter Termin ist notwendig.
- broken: Der Reparatur-Versuch war erfolglos.

## Repair-Cafes (cafe)

Die Repair-Cafe Objekte haben die folgenden Eigenschaften:

- location: Der Ort an dem das Repair-Cafe stattfindet, als einfacher Text.
- address: Die Adresse an der das Repair-Cafe stattfindet, als einfacher Text.
- date: Das Datum an dem das Repair-Cafe stattfindet.

## Geräte (device)

Die Geräte haben die folgenden Eigenschaften:

- date: Das Datum der Geräteanmeldung.
- device: Die Gerätebezeichnung, als einfacher Text.
- manufacturer: Der Gerätehersteller, als einfacher Text.
- issue: Die Fehlerbeschreibung, als formatierter Text.
- picture: Ein Foto des Gerätes.
- type_plate: Ein Foto des Typenschildes des Gerätes.
- confirmed: Boolesches Feld. True falls dem Gast bereits eine Anmeldebestätigung gesendet wurde.
- guest: Fremdschlüssel. Referenz zum Gerätebesitzer.
- state: Fremdschlüssel. Referenz zum Status der Geräteanmeldung.

## Gast (guest)

Die Gäste haben die folgenden Eigenschaften:

- name: Der Name des Gastes, als einfacher Text.
- phone: Die Telefonnummer des Gastes, als einfacher Text.
- residence: Der Wohnort des Gastes, als einfacher Text.
- user: Fremdschlüssel. Referenz zum Benutzer Model. Jeder Gast bekommt einen Benutzerzugang, um seine Daten und die seiner Geräte zu verwalten.

## Helfer (supporter)

Die Helfer haben die folgenden Eigenschaften:

- name: Der Name des Helfers, als einfacher Text.
- user: Fremdschlüssel. Referenz zum Benutzer Model.

## Hilfsobjekte

### Benutzer (user)

Die Benutzer haben die folgenden Eigenschaften:

- mail: Die E-Mail-Adresse des Benutzers.
- notifications: Boolescher Wert. True bedeutet dass der Benutzer über Änderungen per E-Mail benachrichtigt werden möchte.

### Termin (meeting)

Die Termine haben die folgenden Eigenschaften:

- cafe: Fremdschlüssel. Referenz zum Repair-Cafe.
- supporter: Fremdschlüssel. Helfer der bei der Reparatur unterstützt.
- device: Fremdschlüssel. Das Gerät das repariert werden soll.
- time: Die Uhrzeit an der das Treffen stattfindet.
- confirmed: Boolescher Wert. True falls der Gast den Termin bestätigt hat.

### Nachricht (message)

Die Nachrichten haben die folgenden Eigenschaften:

- message: Inhalt der Nachricht, als formatierter Text.
- parent: Fremdschlüssel: Referenz zur vorherigen Nachricht in diesem Kommunikationsbaum.
- date: Das Datum an dem die Nachricht gesendet oder empfangen wurde.
- device: Fremdschlüssel. Referenz zum Gerät über das kommuniziert wird.
- sender: Fremdschlüssel. Referenz zum Benutzer der die Nachricht erstellt hat.

### Status (state)

Die Stati haben die folgenden Eigenschaften:

- name: Eindeutiger Name des Status, als einfacher Text.
- description: Beschreibung des Status, als formatierter Text.
- final: Boolescher Wert. True falls es sich um einen finalen Status handelt.

### Datenmodel

Insgesamt sieht das Datenmodel für die RepOrg Datenbank folgendermaßen aus:

<img src="/datamodel.drawio.svg" width="100%" />

## SQL

Für die Test-Umgebung verwendet RepOrg eine SQLite Datenbank, für die Produktiv-Umgebung verwendet RepOrg eine PostgreSQL Datenbank.

Ziel der SQL Statements ist es datenbank-agnostisch zu sein, da wir nicht erwarten dass das System einer so hohen Last ausgesetzt wird dass die datenbank-spezifische Optimierung notwendig ist, und wir deshalb dei Flexibilität bevorzugen.

<!--
TODO: Wo sind die SQL Statements?
-->