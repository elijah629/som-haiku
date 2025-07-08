# The Great Haiku Search

I wrote this script with `@Emir David Bruges Belaides` to analyze all the
Hackclub server member IDs and find out which ones, just by mentioning them,
make the haiku bot make a haiku out of their name!

## WASM

For demo purposes, this crate has been ported to WASM and a static site. Compile
the demo wasm it yourself with:

```sh
wasm-pack build --release --no-pack --weak-refs -d demo/src/pkg
```

All the flags have been preconfigured in the `Cargo.toml`

## DIY

The below output was created with the following `nushell` script:

```nushell
# First download the two CSVs from slack. Select Name and User ID.
# You can use ID1 and ID2 in /raw for this, sourced  Jul 7, 2025

# Combine them into one
let allUsers = open ID1.csv | append (open ID2.csv)
let allIDs = $allUsers | get "User ID"

# You do not need release mode!
# This code is so optimized that release mode just makes loading the buffer faster
let haikus = $allIDs | to text | cargo run -r

## If you do not want names, $haikus has just the IDs
## If you do want names:
### This may take a while! Nushell lazy evals everything
$haikus |
  lines |
  each {|i| $allUsers | where "User ID" == $i | select Name 'User ID'} |
  flatten |
  to csv
```

## Results for the list of Hackclub server members as of `Mon, Jul 7, 2025`

If you are one of these people: Congrats! If you are not: Don't go ping them
unless you are in a thread and select "Reply Anyway" instead of "Share Thread".
~~They must also not be in the channel.~~ (helps)

```csv
Name,User ID
1enrique3,U0946705BCG
640324,U0920148CV8
Abdalllah Ramadan,U0391602BNE
Abdul Wasay,U09233X915X
Abdullah Saqib,U0823HV759T
Abel Malzew,U0838EL1309
Abhinav Ajith,U0799M813QV
adhithiya650,U0837515STT
AGROCRYPTO,U086LJ3799B
Alba,U079762H5QB
Alex Forman,U0824L493EF
alfredysadonis40,U0870KWG796
annsvetlanam09,U0823V689K9
bakedpotato1029,U0823F859J6
caoanhtai2702,U093746FH7T
Carmelo Kyles,U08240UR719
Cindy Zhao,U0780V477JL
Devah Schaefers,U0410331V2M
Dewan Protiva (she/her),U0203M9445V
Dishita,U08830304SW
dominicjsoutwell,U0925G472UR
Eniola Olawore,U0863PCR737
Erin,U092757EJBT
Evelyn Qiao,U0852A961D3
fahamkhatri777,U0831FG279S
Femil Savaliya,U0333750JSF
Gabrijel,U092770LB5X
ghostlest330,U0825E353EH
Gosakan,U0819951VGC
haarisarshad194,U0828FS787N
hannah6347k,U0844S8350S
jannati,U0874S535GW
Joey M,U0615830F1V
JOSH,U0931K419HU
Joshua Yoo,U0929Q523B5
Kaputt4,U0919J395UP
Karam El labadie,U0836L548T0
khushaliduhoon008,U0941CG443E
Kshitij Verma,U0920FA271U
Leander Riefel,U0917HPK771
M Palmer,U093778NZDY
mason26611,U0825U126JE
matterider,U0809AG2740
matthew58izaguirre,U0825RX572P
Max Allgaier11,U0917K3118F
Mayon Dragon,U0893112AQM
milkyway218,U0810858B5L
Mrigaank,U0947G817V2
mucyo prince,U0899BQA830
Mukul,U0874TG137T
Mukund MV,U0935NG8205
Myroslav Tryhubets,U0924EU5404
nickvalverd,U0851GT159A
Nicola,U0789RAF814
Nihar Phanse,U093768RV6U
Niko,U07960MD940
Noah Intros,U0842T126U9
Param Thawani,U0944UTB457
Pedro Lopez,U0828F9U760
Pritam,U0884S09960
q14bhutton,U0825TKN127
qwemnb1495,U0921Q445RC
raghuwanshiaditya160,U0825TT8403
Raufu Abdulrahman,U0928S977TM
Riswana E A,U01D2806308
rizvi4600779,U0872GPC892
rohitsumbrui,U0918519V6K
s1carnevalepma,U0920580W20
salma,U0921MGV491
samimmud000,U0922L3V537
Sharat Kondapally,U0833G893N3
Shriram Mange,U0922GJ7909
Sophie Möller,U0922708ESV
Spark,U094728N0BT
Sudhakara,U0908441D3P
swatiduck13,U0946EG255X
Taha,U0822B687PU
Taheir Ahmed Ashraf,U0239AG449L
Tobi,U0898Q583C3
yashdharme6,U092755KUMB
Đa Vương Nguyên,U0922EHA879
```
