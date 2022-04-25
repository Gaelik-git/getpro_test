# Un problème de psychométrie

## Énoncé

L’équipe Science souhaite faire passer le Questionnaire Big Five GetPro du format Likert (renseigner à quel point une phrase nous décrit en allant de “Tout à fait d’accord” à “Pas du tout d’accord”, il s’agit d’une échelle sur 5) au format ipsatif (choisir entre différentes phrases, celle qui nous correspond le plus, puis la seconde qui nous décrit le mieux etc. il s’agit d’un classement).  
Pour cela, l’équipe Science doit apparier les items (phrases) du questionnaire sous forme de quadruplets (quatre phrases sont présentées simultanément au candidat).  
Doc a donc informé l’équipe Science que la résolution de ce problème était possible programmatiquement.  
Vous venez d’être recruté chez GetPro et votre première tâche est donc d’aider l’équipe Science à résoudre leur problème.

## Données

### (i) Généralités

Le questionnaire est constitué de :
- 5 domaines (Agréabilité, Conscienciosité, Extraversion, Névrotisme, Ouverture)
- 30 facettes appartenant toutes à un domaine, soit 6 facettes par domaine (A1, A2, A3, A4, A5, A6, C1 … O4, O5, O6)
- 300 phrases/items appartenant à une facette (et donc à un domaine), soit 10 par facette (A1.0, A1.1, … O6.7, O6.8, O6.9)
  
**L’objectif de l’appariement est de réunir les 300 items au sein de 75 quadruplets (75 = 300 items / 4) permettant de comparer chaque facette à toutes les autres.**  
Un quadruplet (par exemple : A1.7 ; C6.0 ; E5.0 ; N4.0)  permet de réaliser 6 comparaisons entre facettes (dans ce cas : A1-C6 ; A1-E5 ; A1-N4 ; C6-E5 ; C6-N4 ; E5-N4). Il y a donc 450 comparaisons réalisées avec 75 quadruplets (75 x 6).  
Idéalement, **chaque facette est comparée une fois à chaque autre facette**, soit 435 comparaisons à réaliser ((30x30)/2 = 450 - 30/2 = 435). Il y aura donc au moins 15 comparaisons redondantes. De plus, on sait que les redondances seront plus nombreuses et qu’il est difficile d’atteindre cet appariement idéal.  
(ii) En ce qui concerne les **domaines**, l’équipe Science veut éviter les quadruplets contenant des items tous issus d’un même domaine. Un quadruplet au format (A,A,A,A) est donc interdit. L’Équipe Science souhaite que les quadruplets obéissent à la répartition suivante qui permet de réaliser le bon nombre de comparaisons entre domaines : 

||||||||
|---|---|---|---|---|---|---|
|x2|x2|x2|x2|x2|x2|x1|
|ACEN|AACE|CCAE|EEAC|NNAC|OOAC|AAA?|
|ACEO|AACN|CCAN|EEAN|NNAE|OOAE|CCC?|
|ACNO|AACO|CCAO|EEAO|NNAO|OOAN|EEE?|
|AENO|AAEN|CCEN|EECN|NNCE|OOCE|NNN?|
|CENO|AAEO|CCEO|EECO|NNCO|OOCN|OOO?|
||AANO|CCNO|EENO|NNEO|OOEN|



(iii) En ce qui concerne les facettes, l’idéal est de comparer chaque facette au moins une fois aux 29 autres (A1 doit être, dans l’idéal, comparée à A2, A3 … O4, O5 et O6). Si cela s’avère impossible, il est possible de ne pas comparer une facette à certaines des autres facettes mais dans ce cas, elle doit au moins être comparée aux autres au sein de son domaine (A1 doit être comparée à A2, A3, A4, A5 et A6). Tout l’objectif de l’appariement est de réaliser un maximum de comparaisons entre facettes. Par ailleurs, il est interdit de comparer une facette à elle-même (A1 ne peut être comparée à A1), il ne faut donc pas d’items issus d’une même facette au sein d’un même quadruplet. En revanche, il est possible de comparer plusieurs fois deux facettes entre elles dans différents quadruplets.

(iv) Les items sont en nombre fixe (300). 

(v) Évaluation
Un bon algorithme permet d’apparier les items selon la répartition présentée dans le tableau. De plus, les facettes d’un même domaine doivent être comparées entre elles autant que possible. Il y a 15 comparaisons à réaliser entre les facettes d’un même domaine (5+4+3+2+1), il est donc possible d’évaluer la qualité de votre solution notamment grâce à ce critère : Nombre de comparaisons différentes réalisées par domaine : 
A = 12/15 ; C = 12/15 ; E = 11/15 ; N = 9/15 ; O = 11/15

Le cas des triplets et des paires peut également être intéressant, n’hésitez pas à vous y intéresser également si vous le souhaitez.

Bon courage !

## (Optionnel) Cas des triplets et des paires
Adaptez votre algorithme au cas des triplets : 100 triplets à réaliser. 
Adaptez votre algorithme au cas des doublets : 150 doublets à réaliser. 
Quelle est la conséquence en termes de comparaisons ? Pensez-vous que l’équipe Science ait bien fait de choisir des quadruplets ? 
