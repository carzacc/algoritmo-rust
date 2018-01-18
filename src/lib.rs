const  quotaCS: u8 = 2.5;

struct Squadra {
    nomesquadra: &str,
    alias: [&str],
    punti: u8,
    puntiTrad: u8,
    golfatti: u16,
    golsubiti: u16,
    sconfitte: u8,
    pareggi: u8,
    vittorie: u8,
    somma: u16,
}

impl Squadra    {
    pub fn new(n: &str, a: [&str]) -> Squadra  {
        Squadra {
            nomesquadra: n,
            alias: a,
            punti: 0,
            puntiTrad: 0,
            golfatti: 0,
            golsubiti: 0,
            sconfitte: 0,
            pareggi: 0,
            vittorie: 0,
            somma: 0,
        }
    }
    pub fn aggiungipartita(&self,GFa: u8, GSa: u8)   {
    if (GFa > GSa) {
      self.puntiTrad = self.puntiTrad + 3;
      self.vittorie = self.vittorie + 1;
    }
    if (GFa == GSa) {
      self.puntiTrad = self.puntiTrad + 1;
      self.pareggi = self.pareggi + 1;
    }
    if (GFa < GSa)  {
      self.sconfitte = self.sconfitte + 1;
    }

    if (GSa == 0) {
      self.punti = self.punti + quotaCS;
    } else {
      if (GSa == 1) {
        self.punti = self.punti + 1.5;
      }
    }
    if (GFa > 0) {
      self.punti = self.punti + 1.3;
    }
    self.golfatti = self.golfatti+GFa;
    self.golsubiti = self.golsubiti+GSa;
  }
  pub fn azzeraPunti(&self)   {
    self.punti=0;
  }
  pub fn azzeraPuntiTrad(&self)   {
    self.puntiTrad=0;
  }
  pub fn resettaGol(&self)    {
    self.golfatti = 0;
    self.golsubiti = 0;
  }
  pub fn resettaPartiteVintePersePareggiate(&self)    {
    self.vittorie = 0;
    self.pareggi = 0;
    self.sconfitte = 0;
  }
  pub fn calcolaSomma(&self)    {
    self.somma = self.punti + self.puntiTrad;
  }
}


/// Ha bisogno che ogni oggetto nell'array squadre abbia nomesquadra e alias come proprietà e aggiungipartita come metodo
pub extern fn partita(squadra1: &str, squadra2: &str, goal1: u8, goal2: u8)  {
  for corrente in squadre  {
    if(corrente.nomesquadra.to_lowercase() == squadra1.to_lowercase())       { corrente.aggiungipartita(goal1,goal2); }
    else if(corrente.nomesquadra.to_lowercase() == squadra2.to_lowercase())  {corrente.aggiungipartita(goal2,goal1);}
    else {
      for al in corrente.alias  {
        if(al.to_lowercase() == squadra1.to_lowercase())       {corrente.aggiungipartita(goal1,goal2);}
        else if(al.to_lowercase() == squadra2.to_lowercase())  {corrente.aggiungipartita(goal2,goal1);}
      }
    }
  }
}

pub extern fn partite (giornata: u8)  {
  let squadre: [Squadra] = [
    Squadra::new("Inter",[]),
    Squadra::new("juventus",["juve"])
  ];

    for squadra in squadre {
    squadra.azzeraPunti();
    squadra.azzeraPuntiTrad();
    squadra.resettaGol();
    squadra.resettaPartiteVintePersePareggiate();
  }
  if !giornata { giornata=40; }
  partita("juventus", "Cagliari", 3, 0);
  partita("Verona", "Napoli", 1, 3);
  partita("Atalanta", "Roma", 0, 1);
  partita("Bologna", "Torino", 1, 1);
  partita("Crotone", "Milan", 0, 3);
  partita("Inter", "Fiorentina", 3, 0);
  partita("Lazio", "Spal", 0, 0);
  partita("Sampdoria", "Benevento", 2, 1);
  partita("Sassuolo", "Genoa", 0, 0);
  partita("Udinese", "Chievo", 1, 2);
  partita("Benevento", "Bologna", 0, 1);
  partita("Genoa", "juventus", 2, 4);
  partita("Roma", "Inter", 1, 3);
  partita("Torino", "Sassuolo", 3, 0);
  partita("Milan", "Cagliari", 2, 1);
  partita("Napoli", "Atalanta", 3, 1);
  partita("Crotone", "Verona", 0, 0);
  partita("Spal", "Udinese", 3, 2);
  partita("Chievo", "Lazio", 1, 2);
  partita("Fiorentina", "Sampdoria", 1, 2);
  partita("juventus", "Chievo", 3, 0);
  partita("Inter", "Spal", 2, 0);
  partita("Verona", "Fiorentina", 0, 5);
  partita("Udinese", "Genoa", 1, 0);
  partita("Atalanta", "Sassuolo", 2, 1);
  partita("Cagliari", "Crotone", 1, 0);
  partita("Lazio", "Milan", 4, 1);
  partita("Benevento", "Torino", 0, 1);
  partita("Bologna", "Napoli", 0, 3);
  partita("Crotone", "Inter", 0, 2);
  partita("Fiorentina", "Bologna", 2, 1);
  partita("Roma", "Verona", 3, 0);
  partita("Sassuolo", "juventus", 1, 3);
  partita("Milan", "Udinese", 2, 1);
  partita("Napoli", "Benevento", 6, 0);
  partita("Spal", "Cagliari", 0, 2);
  partita("Torino", "Sampdoria", 2, 2);
  partita("Chievo", "Atalanta", 1, 1);
  partita("Genoa", "Lazio", 2, 3);
  if giornata>4 {
    partita("Bologna", "Inter", 1, 1);
    partita("Benevento", "Roma", 0, 4);
    partita("Atalanta", "Crotone", 5, 1);
    partita("Cagliari", "Sassuolo", 0, 1);
    partita("Genoa", "Chievo", 1, 1);
    partita("juventus", "Fiorentina", 1, 0);
    partita("Lazio", "Napoli", 1, 4);
    partita("Milan", "Spal", 2, 0);
    partita("Udinese", "Torino", 2, 3);
    partita("Verona", "Sampdoria", 0, 0);
    if giornata>5 {
      partita("Roma", "Udinese", 3, 1);
      partita("Spal", "Napoli", 2, 3);
      partita("juventus", "Torino", 4, 0);
      partita("Sampdoria", "Milan", 2, 0);
      partita("Cagliari", "Chievo", 0, 2);
      partita("Crotone", "Benevento", 2, 0);
      partita("Verona", "Lazio", 0, 3);
      partita("Inter", "Genoa", 1, 0);
      partita("Sassuolo", "Bologna", 0, 1);
      partita("Fiorentina", "Atalanta", 1, 1);
    }
    if giornata>6
    {
      partita("Udinese","Sampdoria",4,0);
      partita("Genoa","Bologna",0,1);
      partita("Napoli","Cagliari",3,0);
      partita("Benevento","Inter",1,2);
      partita("Lazio","Sassuolo",6,1);
      partita("Torino","Verona",2,2);
      partita("Chievo","Fiorentina",2,1);
      partita("Spal","Crotone",1,1);
      partita("Milan","Roma",0,2);
      partita("Atalanta","juventus",2,2);
    }
    if giornata>7
    {
      partita("juventus","Lazio",1,2);
      partita("Roma","Napoli",0,1);
      partita("Fiorentina","Udinese",2,1);
      partita("Bologna","Spal",2,1);
      partita("Cagliari","Genoa",2,3);
      partita("Crotone","Torino",2,2);
      partita("Sampdoria", "Atalanta",3,1);
      partita("Sassuolo","Chievo",0,0);
      partita("Inter","Milan",3,2);
      partita("Verona", "Benevento",1,0);
      if giornata>8
      {
        partita("Sampdoria","Crotone",5,0);
        partita("Napoli","Inter",0,0);
        partita("Chievo","Verona",3,2);
        partita("Atalanta","Bologna",1,0);
        partita("Benevento", "Fiorentina",0,3);
        partita("Milan","Genoa",0,0);
        partita("Spal","Sassuolo",0,1);
        partita("Torino","Roma",0,1);
        partita("Udinese","juventus",2,6);
        partita("Lazio","Cagliari",3,0);
        if giornata>9
        {
          partita("Inter","Sampdoria",3,2);
          partita("Atalanta","Verona",3,0);
          partita("Bologna","Lazio",1,2);
          partita("Cagliari", "Benevento",2,1);
          partita("Chievo","Milan",1,4);
          partita("Fiorentina","Torino",3,0);
          partita("Genoa","Napoli",2,3);
          partita("juventus","Spal",4,1);
          partita("Roma","Crotone",1,0);
          partita("Sassuolo","Udinese",0,1);
          if giornata>10 {
            partita("Milan","juventus",0,2);
            partita("Roma","Bologna",1,0);
            partita("Benevento","Lazio",1,5);
            partita("Sampdoria","Chievo",4,1);
            partita("Spal","Genoa",1,0);
            partita("Napoli","Sassuolo",3,1);
            partita("Udinese","Atalanta",2,1);
            partita("Crotone", "Fiorentina",2,1);
            partita("Torino","Cagliari",2,1);
            partita("Verona","Inter",1,2);
          }
          if giornata>11 {
            partita("Bologna","Crotone",2,3);
            partita("Genoa","Sampdoria",0,2);
            partita("Inter","Torino",1,1);
            partita("Fiorentina","Roma",2,4);
            partita("Cagliari","Verona",2,1);
            partita("juventus","Benevento",2,1);
            partita("Chievo","Napoli",0,0);
            partita("Atalanta","Spal",1,1);
            partita("Sassuolo","Milan",0,2);
          }
          if giornata>12 {
            partita("Roma","Lazio",2,1);
            partita("Napoli","Milan",2,1);
            partita("Crotone","Genoa",0,1);
            partita("Benevento","Sassuolo",1,2);
            partita("Sampdoria","juventus",3,2);
            partita("Spal","Fiorentina",1,1);
            partita("Torino","Chievo",1,1);
            partita("Udinese","Cagliari",0,1);
            partita("Inter","Atalanta",2,0);
            partita("Verona","Bologna",2,3);
          }
          if giornata>13  {
            partita("Bologna","Sampdoria",3,0);
            partita("Sassuolo","Verona",0,2);
            partita("Chievo","Spal",2,1);
            partita("Cagliari","Inter",1,3);
            partita("Milan","Torino",0,0);
            partita("Genoa","Roma",1,1);
            partita("Udinese","Napoli",0,1);
            partita("Lazio","Fiorentina",1,1);
            partita("juventus","Crotone",3,0);
            partita("Atalanta","Benevento",1,0);
          }
          if giornata>14  {
            partita("Roma","Spal",3,1);
            partita("Napoli","juventus",0,1);
            partita("Torino","Atalanta",1,1);
            partita("Benevento","Milan",2,2);
            partita("Bologna","Cagliari",1,1);
            partita("Fiorentina","Sassuolo",3,0);
            partita("Inter","Chievo",5,0);
            partita("Sampdoria","Lazio",1,2);
            partita("Crotone","Udinese",0,3);
            partita("Verona","Genoa",0,1);
          }
          if giornata>15 {
            partita("Cagliari","Sampdoria",2,2);
            partita("juventus","Inter",0,0);
            partita("Chievo","Roma",0,0);
            partita("Napoli","Fiorentina",0,0);
            partita("Spal","Verona",2,2);
            partita("Udinese","Benevento",2,0);
            partita("Sassuolo","Crotone",2,1);
            partita("Milan","Bologna",2,1);
            partita("Genoa","Atalanta",1,2);
            partita("Lazio","Torino",1,3);
          }
          if giornata>16 {
            partita("Inter","Udinese",1,3);
            partita("Torino","Napoli",1,3);
            partita("Roma","Cagliari",1,0);
            partita("Verona","Milan",3,0);
            partita("Fiorentina","Genoa",0,0);
            partita("Sampdoria","Sassuolo",0,1);
            partita("Bologna","Juventus",0,3);
            partita("Crotone","Chievo",1,0);
            partita("Benevento","Spal",1,2);
            partita("Atalanta","Lazio",3,3);
          }
          if giornata>17 {
            partita("Chievo","Bologna",2,3);
            partita("Cagliari","Fiorentina",0,1);
            partita("Lazio","Crotone",4,0);
            partita("Sassuolo","Inter",1,0);
            partita("Udinese","Verona",4,0);
            partita("Napoli","Sampdoria",3,2);
            partita("Spal","Torino",2,2);
            partita("Genoa","Benevento",1,0);
            partita("Milan","Atalanta",0,2);
            partita("Juventus","Roma",1,0);
          }
          if giornata>18 {
            partita("Crotone","Napoli",0,1);
            partita("Fiorentina","Milan",1,1);
            partita("Torino","Genoa",0,0);
            partita("Benevento","Chievo",1,0);
            partita("Roma","Sassuolo",1,1);
            partita("Sampdoria","Spal",2,0);
            partita("Atalanta","Cagliari",1,2);
            partita("Bologna","Udinese",1,2);
            partita("Inter","Lazio",0,0);
            partita("Verona","Juventus",1,3);
          }
          if giornata>19 {
            partita("Chievo","Udinese",1,1);
            partita("Fiorentina","Inter",1,1);
            partita("Torino","Bologna",3,0);
            partita("Milan","Crotone",1,0);
            partita("Benevento","Sampdoria",3,2);
            partita("Genoa","Sassuolo",1,0);
            partita("Spal","Lazio",2,5);
            partita("Napoli","Verona",2,0);
            partita("Roma","Atalanta",1,2);
            partita("Cagliari","juventus",0,1);
          }
        }
      }
    }
  }
  for s in squadre { s.calcolaSomma(); }
}

