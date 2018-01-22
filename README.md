# Work in progress!
Sarà un'implementazione Rust di http://algorest.carzacc.info

# LE DUE VERSIONI

Ci sono due branch:
`web` dove ci sarà un'implementazione come server per il Web
`cli` che restitirà la classifica nella riga di comando

## TODO
 Il problemi sono
+ le squadre non sono in ordine di classifica(secondario)
+ i campi per ogni squadra sono ordinati in ordine alfabetico per nome del campo

L'output che si ottiene dal programma  è questo


 ``` json
[
  {
    "Alternativa": 55.29999923706055,
    "Gol Fatti": 35,
    "Gol Subiti": 15,
    "Pareggi": 6,
    "Sconfitte": 2,
    "Somma": 97.30000305175781,
    "Squadra": "Inter",
    "Tradizionale": 42,
    "Vittorie": 12
  },
  {
    "Alternativa": 58.20000076293945,
    "Gol Fatti": 49,
    "Gol Subiti": 15,
    "Pareggi": 2,
    "Sconfitte": 2,
    "Somma": 108.19999694824219,
    "Squadra": "juventus",
    "Tradizionale": 50,
    "Vittorie": 16
  },
  {
    "Alternativa": 35.099998474121094,
    "Gol Fatti": 18,
    "Gol Subiti": 31,
    "Pareggi": 2,
    "Sconfitte": 12,
    "Somma": 55.099998474121094,
    "Squadra": "cagliari",
    "Tradizionale": 20,
    "Vittorie": 6
  },
  {
    "Alternativa": 28.299999237060547,
    "Gol Fatti": 18,
    "Gol Subiti": 41,
    "Pareggi": 4,
    "Sconfitte": 13,
    "Somma": 41.29999923706055,
    "Squadra": "verona",
    "Tradizionale": 13,
    "Vittorie": 3
  },
  {
    "Alternativa": 56.29999923706055,
    "Gol Fatti": 44,
    "Gol Subiti": 13,
    "Pareggi": 3,
    "Sconfitte": 1,
    "Somma": 107.30000305175781,
    "Squadra": "napoli",
    "Tradizionale": 51,
    "Vittorie": 16
  },
  {
    "Alternativa": 42.5,
    "Gol Fatti": 28,
    "Gol Subiti": 27,
    "Pareggi": 10,
    "Sconfitte": 4,
    "Somma": 70.5,
    "Squadra": "torino",
    "Tradizionale": 28,
    "Vittorie": 6
  },
  {
    "Alternativa": 46.900001525878906,
    "Gol Fatti": 31,
    "Gol Subiti": 26,
    "Pareggi": 6,
    "Sconfitte": 6,
    "Somma": 76.9000015258789,
    "Squadra": "atalanta",
    "Tradizionale": 30,
    "Vittorie": 8
  },
  {
    "Alternativa": 38.5,
    "Gol Fatti": 23,
    "Gol Subiti": 30,
    "Pareggi": 3,
    "Sconfitte": 10,
    "Somma": 62.5,
    "Squadra": "bologna",
    "Tradizionale": 24,
    "Vittorie": 7
  },
  {
    "Alternativa": 39.400001525878906,
    "Gol Fatti": 25,
    "Gol Subiti": 27,
    "Pareggi": 4,
    "Sconfitte": 8,
    "Somma": 67.4000015258789,
    "Squadra": "milan",
    "Tradizionale": 28,
    "Vittorie": 8
  },
  {
    "Alternativa": 28.399999618530273,
    "Gol Fatti": 13,
    "Gol Subiti": 38,
    "Pareggi": 3,
    "Sconfitte": 13,
    "Somma": 43.400001525878906,
    "Squadra": "crotone",
    "Tradizionale": 15,
    "Vittorie": 4
  },
  {
    "Alternativa": 30.700000762939453,
    "Gol Fatti": 21,
    "Gol Subiti": 38,
    "Pareggi": 6,
    "Sconfitte": 11,
    "Somma": 45.70000076293945,
    "Squadra": "spal",
    "Tradizionale": 15,
    "Vittorie": 3
  },
  {
    "Alternativa": 43.29999923706055,
    "Gol Fatti": 16,
    "Gol Subiti": 22,
    "Pareggi": 6,
    "Sconfitte": 9,
    "Somma": 64.30000305175781,
    "Squadra": "genoa",
    "Tradizionale": 21,
    "Vittorie": 5
  },
  {
    "Alternativa": 40.400001525878906,
    "Gol Fatti": 20,
    "Gol Subiti": 32,
    "Pareggi": 7,
    "Sconfitte": 8,
    "Somma": 62.400001525878906,
    "Squadra": "chievo",
    "Tradizionale": 22,
    "Vittorie": 5
  },
  {
    "Alternativa": 53.79999923706055,
    "Gol Fatti": 30,
    "Gol Subiti": 14,
    "Pareggi": 3,
    "Sconfitte": 4,
    "Somma": 92.80000305175781,
    "Squadra": "roma",
    "Tradizionale": 39,
    "Vittorie": 12
  },
  {
    "Alternativa": 38.29999923706055,
    "Gol Fatti": 13,
    "Gol Subiti": 30,
    "Pareggi": 3,
    "Sconfitte": 11,
    "Somma": 59.29999923706055,
    "Squadra": "sassuolo",
    "Tradizionale": 21,
    "Vittorie": 6
  },
  {
    "Alternativa": 50.29999923706055,
    "Gol Fatti": 29,
    "Gol Subiti": 21,
    "Pareggi": 7,
    "Sconfitte": 6,
    "Somma": 78.30000305175781,
    "Squadra": "fiorentina",
    "Tradizionale": 28,
    "Vittorie": 7
  },
  {
    "Alternativa": 23,
    "Gol Fatti": 13,
    "Gol Subiti": 43,
    "Pareggi": 1,
    "Sconfitte": 17,
    "Somma": 30,
    "Squadra": "benevento",
    "Tradizionale": 7,
    "Vittorie": 2
  },
  {
    "Alternativa": 46.599998474121094,
    "Gol Fatti": 48,
    "Gol Subiti": 24,
    "Pareggi": 4,
    "Sconfitte": 3,
    "Somma": 86.5999984741211,
    "Squadra": "lazio",
    "Tradizionale": 40,
    "Vittorie": 12
  },
  {
    "Alternativa": 39.5,
    "Gol Fatti": 36,
    "Gol Subiti": 29,
    "Pareggi": 3,
    "Sconfitte": 7,
    "Somma": 69.5,
    "Squadra": "sampdoria",
    "Tradizionale": 30,
    "Vittorie": 9
  },
  {
    "Alternativa": 46.099998474121094,
    "Gol Fatti": 33,
    "Gol Subiti": 27,
    "Pareggi": 1,
    "Sconfitte": 9,
    "Somma": 74.0999984741211,
    "Squadra": "udinese",
    "Tradizionale": 28,
    "Vittorie": 9
  }
]
```