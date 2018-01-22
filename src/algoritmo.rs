pub mod algoritmo 
{
  const  QUOTA_CS: f32 = 2.5;
  const NUMERO_SQUADRE: usize = 20;
  pub struct Squadra {
      pub nomesquadra: String,
      pub punti: f32,
      pub punti_trad: u16,
      pub golfatti: u16,
      pub golsubiti: u16,
      pub sconfitte: u16,
      pub pareggi: u16,
      pub vittorie: u16,
  }

  impl Squadra    {
      pub  fn new(n: &str) -> Squadra  {
          Squadra {
              nomesquadra: n.to_owned(),
              punti: 0.0,
              punti_trad: 0,
              golfatti: 0,
              golsubiti: 0,
              sconfitte: 0,
              pareggi: 0,
              vittorie: 0,
          }
      }
      pub  fn somma(&self) -> f32  {
        self.punti + (self.punti_trad as f32)
      }
      pub  fn aggiungipartita(&mut self,gf: u16, gs: u16)   {
      if (gf > gs) {
        self.punti_trad = self.punti_trad + 3;
        self.vittorie = self.vittorie + 1;
      }
      if (gf == gs) {
        self.punti_trad = self.punti_trad + 1;
        self.pareggi = self.pareggi + 1;
      }
      if (gf < gs)  {
        self.sconfitte = self.sconfitte + 1;
      }

      if (gs == 0) {
        self.punti = self.punti + QUOTA_CS;
      } else {
        if (gs == 1) {
          self.punti = self.punti + 1.5;
        }
      }  

      if (gf > 0) {
        self.punti = self.punti + 1.3;
      }
      self.golfatti = self.golfatti+gf;
      self.golsubiti = self.golsubiti+gs;
    }
  }
  pub fn arrotonda(x: f32) -> f32 { ((x*10.0).round())/10.0 }

  /// Ha bisogno che ogni oggetto nell'array squadre abbia nomesquadra e alias come proprietà e aggiungipartita come metodo
  pub fn partita(ref mut s: &mut [Squadra; NUMERO_SQUADRE], squadra1: &str, squadra2: &str, goal1: u16, goal2: u16)  {
    for a in 0..NUMERO_SQUADRE  {
      if(s[a].nomesquadra.to_lowercase() == squadra1.to_lowercase())       {s[a].aggiungipartita(goal1,goal2);}
      if(s[a].nomesquadra.to_lowercase() == squadra2.to_lowercase())  {s[a].aggiungipartita(goal2,goal1);}
    }
  }

  pub fn calcola_algoritmo (giornata: u8) -> [Squadra; NUMERO_SQUADRE]  {
    let mut squadre: [Squadra; NUMERO_SQUADRE] = [
      Squadra::new("Inter"),
      Squadra::new("juventus"),
      Squadra::new("cagliari"),
      Squadra::new("verona"),
      Squadra::new("napoli"),
      Squadra::new("torino"),
      Squadra::new("atalanta"),
      Squadra::new("bologna"),
      Squadra::new("milan"),
      Squadra::new("crotone"),
      Squadra::new("spal"),
      Squadra::new("genoa"),
      Squadra::new("chievo"),
      Squadra::new("roma"),
      Squadra::new("sassuolo"),
      Squadra::new("fiorentina"),
      Squadra::new("benevento"),
      Squadra::new("lazio"),
      Squadra::new("sampdoria"),
      Squadra::new("udinese")
    ];
    partita(&mut squadre, "juventus", "Cagliari", 3, 0);
    partita(&mut squadre, "Verona", "Napoli", 1, 3);
    partita(&mut squadre, "Atalanta", "Roma", 0, 1);
    partita(&mut squadre, "Bologna", "Torino", 1, 1);
    partita(&mut squadre, "Crotone", "Milan", 0, 3);
    partita(&mut squadre, "Inter", "Fiorentina", 3, 0);
    partita(&mut squadre, "Lazio", "Spal", 0, 0);
    partita(&mut squadre, "Sampdoria", "Benevento", 2, 1);
    partita(&mut squadre, "Sassuolo", "Genoa", 0, 0);
    partita(&mut squadre, "Udinese", "Chievo", 1, 2);
    partita(&mut squadre, "Benevento", "Bologna", 0, 1);
    partita(&mut squadre, "Genoa", "juventus", 2, 4);
    partita(&mut squadre, "Roma", "Inter", 1, 3);
    partita(&mut squadre, "Torino", "Sassuolo", 3, 0);
    partita(&mut squadre, "Milan", "Cagliari", 2, 1);
    partita(&mut squadre, "Napoli", "Atalanta", 3, 1);
    partita(&mut squadre, "Crotone", "Verona", 0, 0);
    partita(&mut squadre, "Spal", "Udinese", 3, 2);
    partita(&mut squadre, "Chievo", "Lazio", 1, 2);
    partita(&mut squadre, "Fiorentina", "Sampdoria", 1, 2);
    partita(&mut squadre, "juventus", "Chievo", 3, 0);
    partita(&mut squadre, "Inter", "Spal", 2, 0);
    partita(&mut squadre, "Verona", "Fiorentina", 0, 5);
    partita(&mut squadre, "Udinese", "Genoa", 1, 0);
    partita(&mut squadre, "Atalanta", "Sassuolo", 2, 1);
    partita(&mut squadre, "Cagliari", "Crotone", 1, 0);
    partita(&mut squadre, "Lazio", "Milan", 4, 1);
    partita(&mut squadre, "Benevento", "Torino", 0, 1);
    partita(&mut squadre, "Bologna", "Napoli", 0, 3);
    partita(&mut squadre, "Crotone", "Inter", 0, 2);
    partita(&mut squadre, "Fiorentina", "Bologna", 2, 1);
    partita(&mut squadre, "Roma", "Verona", 3, 0);
    partita(&mut squadre, "Sassuolo", "juventus", 1, 3);
    partita(&mut squadre, "Milan", "Udinese", 2, 1);
    partita(&mut squadre, "Napoli", "Benevento", 6, 0);
    partita(&mut squadre, "Spal", "Cagliari", 0, 2);
    partita(&mut squadre, "Torino", "Sampdoria", 2, 2);
    partita(&mut squadre, "Chievo", "Atalanta", 1, 1);
    partita(&mut squadre, "Genoa", "Lazio", 2, 3);
    if giornata>4 {
      partita(&mut squadre, "Bologna", "Inter", 1, 1);
      partita(&mut squadre, "Benevento", "Roma", 0, 4);
      partita(&mut squadre, "Atalanta", "Crotone", 5, 1);
      partita(&mut squadre, "Cagliari", "Sassuolo", 0, 1);
      partita(&mut squadre, "Genoa", "Chievo", 1, 1);
      partita(&mut squadre, "juventus", "Fiorentina", 1, 0);
      partita(&mut squadre, "Lazio", "Napoli", 1, 4);
      partita(&mut squadre, "Milan", "Spal", 2, 0);
      partita(&mut squadre, "Udinese", "Torino", 2, 3);
      partita(&mut squadre, "Verona", "Sampdoria", 0, 0);
      if giornata>5 {
        partita(&mut squadre, "Roma", "Udinese", 3, 1);
        partita(&mut squadre, "Spal", "Napoli", 2, 3);
        partita(&mut squadre, "juventus", "Torino", 4, 0);
        partita(&mut squadre, "Sampdoria", "Milan", 2, 0);
        partita(&mut squadre, "Cagliari", "Chievo", 0, 2);
        partita(&mut squadre, "Crotone", "Benevento", 2, 0);
        partita(&mut squadre, "Verona", "Lazio", 0, 3);
        partita(&mut squadre, "Inter", "Genoa", 1, 0);
        partita(&mut squadre, "Sassuolo", "Bologna", 0, 1);
        partita(&mut squadre, "Fiorentina", "Atalanta", 1, 1);
      }
      if giornata>6
      {
        partita(&mut squadre, "Udinese","Sampdoria",4,0);
        partita(&mut squadre, "Genoa","Bologna",0,1);
        partita(&mut squadre, "Napoli","Cagliari",3,0);
        partita(&mut squadre, "Benevento","Inter",1,2);
        partita(&mut squadre, "Lazio","Sassuolo",6,1);
        partita(&mut squadre, "Torino","Verona",2,2);
        partita(&mut squadre, "Chievo","Fiorentina",2,1);
        partita(&mut squadre, "Spal","Crotone",1,1);
        partita(&mut squadre, "Milan","Roma",0,2);
        partita(&mut squadre, "Atalanta","juventus",2,2);
      }
      if giornata>7
      {
        partita(&mut squadre, "juventus","Lazio",1,2);
        partita(&mut squadre, "Roma","Napoli",0,1);
        partita(&mut squadre, "Fiorentina","Udinese",2,1);
        partita(&mut squadre, "Bologna","Spal",2,1);
        partita(&mut squadre, "Cagliari","Genoa",2,3);
        partita(&mut squadre, "Crotone","Torino",2,2);
        partita(&mut squadre, "Sampdoria", "Atalanta",3,1);
        partita(&mut squadre, "Sassuolo","Chievo",0,0);
        partita(&mut squadre, "Inter","Milan",3,2);
        partita(&mut squadre, "Verona", "Benevento",1,0);
        if giornata>8
        {
          partita(&mut squadre, "Sampdoria","Crotone",5,0);
          partita(&mut squadre, "Napoli","Inter",0,0);
          partita(&mut squadre, "Chievo","Verona",3,2);
          partita(&mut squadre, "Atalanta","Bologna",1,0);
          partita(&mut squadre, "Benevento", "Fiorentina",0,3);
          partita(&mut squadre, "Milan","Genoa",0,0);
          partita(&mut squadre, "Spal","Sassuolo",0,1);
          partita(&mut squadre, "Torino","Roma",0,1);
          partita(&mut squadre, "Udinese","juventus",2,6);
          partita(&mut squadre, "Lazio","Cagliari",3,0);
          if giornata>9
          {
            partita(&mut squadre, "Inter","Sampdoria",3,2);
            partita(&mut squadre, "Atalanta","Verona",3,0);
            partita(&mut squadre, "Bologna","Lazio",1,2);
            partita(&mut squadre, "Cagliari", "Benevento",2,1);
            partita(&mut squadre, "Chievo","Milan",1,4);
            partita(&mut squadre, "Fiorentina","Torino",3,0);
            partita(&mut squadre, "Genoa","Napoli",2,3);
            partita(&mut squadre, "juventus","Spal",4,1);
            partita(&mut squadre, "Roma","Crotone",1,0);
            partita(&mut squadre, "Sassuolo","Udinese",0,1);
            if giornata>10 {
              partita(&mut squadre, "Milan","juventus",0,2);
              partita(&mut squadre, "Roma","Bologna",1,0);
              partita(&mut squadre, "Benevento","Lazio",1,5);
              partita(&mut squadre, "Sampdoria","Chievo",4,1);
              partita(&mut squadre, "Spal","Genoa",1,0);
              partita(&mut squadre, "Napoli","Sassuolo",3,1);
              partita(&mut squadre, "Udinese","Atalanta",2,1);
              partita(&mut squadre, "Crotone", "Fiorentina",2,1);
              partita(&mut squadre, "Torino","Cagliari",2,1);
              partita(&mut squadre, "Verona","Inter",1,2);
            }
            if giornata>11 {
              partita(&mut squadre, "Bologna","Crotone",2,3);
              partita(&mut squadre, "Genoa","Sampdoria",0,2);
              partita(&mut squadre, "Inter","Torino",1,1);
              partita(&mut squadre, "Fiorentina","Roma",2,4);
              partita(&mut squadre, "Cagliari","Verona",2,1);
              partita(&mut squadre, "juventus","Benevento",2,1);
              partita(&mut squadre, "Chievo","Napoli",0,0);
              partita(&mut squadre, "Atalanta","Spal",1,1);
              partita(&mut squadre, "Sassuolo","Milan",0,2);
            }
            if giornata>12 {
              partita(&mut squadre, "Roma","Lazio",2,1);
              partita(&mut squadre, "Napoli","Milan",2,1);
              partita(&mut squadre, "Crotone","Genoa",0,1);
              partita(&mut squadre, "Benevento","Sassuolo",1,2);
              partita(&mut squadre, "Sampdoria","juventus",3,2);
              partita(&mut squadre, "Spal","Fiorentina",1,1);
              partita(&mut squadre, "Torino","Chievo",1,1);
              partita(&mut squadre, "Udinese","Cagliari",0,1);
              partita(&mut squadre, "Inter","Atalanta",2,0);
              partita(&mut squadre, "Verona","Bologna",2,3);
            }
            if giornata>13  {
              partita(&mut squadre, "Bologna","Sampdoria",3,0);
              partita(&mut squadre, "Sassuolo","Verona",0,2);
              partita(&mut squadre, "Chievo","Spal",2,1);
              partita(&mut squadre, "Cagliari","Inter",1,3);
              partita(&mut squadre, "Milan","Torino",0,0);
              partita(&mut squadre, "Genoa","Roma",1,1);
              partita(&mut squadre, "Udinese","Napoli",0,1);
              partita(&mut squadre, "Lazio","Fiorentina",1,1);
              partita(&mut squadre, "juventus","Crotone",3,0);
              partita(&mut squadre, "Atalanta","Benevento",1,0);
            }
            if giornata>14  {
              partita(&mut squadre, "Roma","Spal",3,1);
              partita(&mut squadre, "Napoli","juventus",0,1);
              partita(&mut squadre, "Torino","Atalanta",1,1);
              partita(&mut squadre, "Benevento","Milan",2,2);
              partita(&mut squadre, "Bologna","Cagliari",1,1);
              partita(&mut squadre, "Fiorentina","Sassuolo",3,0);
              partita(&mut squadre, "Inter","Chievo",5,0);
              partita(&mut squadre, "Sampdoria","Lazio",1,2);
              partita(&mut squadre, "Crotone","Udinese",0,3);
              partita(&mut squadre, "Verona","Genoa",0,1);
            }
            if giornata>15 {
              partita(&mut squadre, "Cagliari","Sampdoria",2,2);
              partita(&mut squadre, "juventus","Inter",0,0);
              partita(&mut squadre, "Chievo","Roma",0,0);
              partita(&mut squadre, "Napoli","Fiorentina",0,0);
              partita(&mut squadre, "Spal","Verona",2,2);
              partita(&mut squadre, "Udinese","Benevento",2,0);
              partita(&mut squadre, "Sassuolo","Crotone",2,1);
              partita(&mut squadre, "Milan","Bologna",2,1);
              partita(&mut squadre, "Genoa","Atalanta",1,2);
              partita(&mut squadre, "Lazio","Torino",1,3);
            }
            if giornata>16 {
              partita(&mut squadre, "Inter","Udinese",1,3);
              partita(&mut squadre, "Torino","Napoli",1,3);
              partita(&mut squadre, "Roma","Cagliari",1,0);
              partita(&mut squadre, "Verona","Milan",3,0);
              partita(&mut squadre, "Fiorentina","Genoa",0,0);
              partita(&mut squadre, "Sampdoria","Sassuolo",0,1);
              partita(&mut squadre, "Bologna","Juventus",0,3);
              partita(&mut squadre, "Crotone","Chievo",1,0);
              partita(&mut squadre, "Benevento","Spal",1,2);
              partita(&mut squadre, "Atalanta","Lazio",3,3);
            }
            if giornata>17 {
              partita(&mut squadre, "Chievo","Bologna",2,3);
              partita(&mut squadre, "Cagliari","Fiorentina",0,1);
              partita(&mut squadre, "Lazio","Crotone",4,0);
              partita(&mut squadre, "Sassuolo","Inter",1,0);
              partita(&mut squadre, "Udinese","Verona",4,0);
              partita(&mut squadre, "Napoli","Sampdoria",3,2);
              partita(&mut squadre, "Spal","Torino",2,2);
              partita(&mut squadre, "Genoa","Benevento",1,0);
              partita(&mut squadre, "Milan","Atalanta",0,2);
              partita(&mut squadre, "Juventus","Roma",1,0);
            }
            if giornata>18 {
              partita(&mut squadre, "Crotone","Napoli",0,1);
              partita(&mut squadre, "Fiorentina","Milan",1,1);
              partita(&mut squadre, "Torino","Genoa",0,0);
              partita(&mut squadre, "Benevento","Chievo",1,0);
              partita(&mut squadre, "Roma","Sassuolo",1,1);
              partita(&mut squadre, "Sampdoria","Spal",2,0);
              partita(&mut squadre, "Atalanta","Cagliari",1,2);
              partita(&mut squadre, "Bologna","Udinese",1,2);
              partita(&mut squadre, "Inter","Lazio",0,0);
              partita(&mut squadre, "Verona","Juventus",1,3);
            }
            if giornata>19 {
              partita(&mut squadre, "Chievo","Udinese",1,1);
              partita(&mut squadre, "Fiorentina","Inter",1,1);
              partita(&mut squadre, "Torino","Bologna",3,0);
              partita(&mut squadre, "Milan","Crotone",1,0);
              partita(&mut squadre, "Benevento","Sampdoria",3,2);
              partita(&mut squadre, "Genoa","Sassuolo",1,0);
              partita(&mut squadre, "Spal","Lazio",2,5);
              partita(&mut squadre, "Napoli","Verona",2,0);
              partita(&mut squadre, "Roma","Atalanta",1,2);
              partita(&mut squadre, "Cagliari","juventus",0,1);
            }
          }
        }
      }
    }
    return squadre
  }
}