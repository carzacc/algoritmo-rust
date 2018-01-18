# Work in progress!
Sarà un'implementazione Rust di http://algorest.carzacc.info

# Errori attuali
error[E0425]: cannot find value `squadre` in this scope
  --> src/lib.rs:80:19
   |
80 |   for corrente in squadre  {
   |                   ^^^^^^^ did you mean `squadra1`?

warning: unnecessary parentheses around `if` condition
  --> src/lib.rs:32:8
   |
32 |     if (GFa > GSa) {
   |        ^^^^^^^^^^^ help: remove these parentheses
   |
   = note: #[warn(unused_parens)] on by default

warning: unnecessary parentheses around `if` condition
  --> src/lib.rs:36:8
   |
36 |     if (GFa == GSa) {
   |        ^^^^^^^^^^^^ help: remove these parentheses

warning: unnecessary parentheses around `if` condition
  --> src/lib.rs:40:8
   |
40 |     if (GFa < GSa)  {
   |        ^^^^^^^^^^^ help: remove these parentheses

warning: unnecessary parentheses around `if` condition
  --> src/lib.rs:44:8
   |
44 |     if (GSa == 0) {
   |        ^^^^^^^^^^ help: remove these parentheses

warning: unnecessary parentheses around `if` condition
  --> src/lib.rs:47:10
   |
47 |       if (GSa == 1) {
   |          ^^^^^^^^^^ help: remove these parentheses

warning: unnecessary parentheses around `if` condition
  --> src/lib.rs:51:8
   |
51 |     if (GFa > 0) {
   |        ^^^^^^^^^ help: remove these parentheses

warning: unnecessary parentheses around `if` condition
  --> src/lib.rs:81:7
   |
81 |     if(corrente.nomesquadra.to_lowercase() == squadra1.to_lowercase())       { corrente.aggiungipartita(goal1,goal2); }
   |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses

warning: unnecessary parentheses around `if` condition
  --> src/lib.rs:82:12
   |
82 |     else if(corrente.nomesquadra.to_lowercase() == squadra2.to_lowercase())  {corrente.aggiungipartita(goal2,goal1);}
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses

warning: unnecessary parentheses around `if` condition
  --> src/lib.rs:85:11
   |
85 |         if(al.to_lowercase() == squadra1.to_lowercase())       {corrente.aggiungipartita(goal1,goal2);}
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses

warning: unnecessary parentheses around `if` condition
  --> src/lib.rs:86:16
   |
86 |         else if(al.to_lowercase() == squadra2.to_lowercase())  {corrente.aggiungipartita(goal2,goal1);}
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses

error[E0106]: missing lifetime specifier
 --> src/lib.rs:4:18
  |
4 |     nomesquadra: &str,
  |                  ^ expected lifetime parameter

error[E0106]: missing lifetime specifier
 --> src/lib.rs:5:13
  |
5 |     alias: [&str],
  |             ^ expected lifetime parameter

error: aborting due to 3 previous errors

error: Could not compile `Algoritmo-rust`.
