++++++++++
++++++++++
++++++++++
++++++++++
++++++++                      a0 = '0'
[
  ->+>>>>>>>>>>
  +>+>+>+>+>+>+>+>+
  <<<<<<<<<<<<<<<<<<<
]                             a0 ) a1 = a11:19
>
[-<+>]                        a1 ) a0
>>>>>>>>>+                    a10 = 1  x tomove
>+                            a11 = '1'
>++                           a12 = '2'
>+++                          a13 = '3'
>++++                         a14 = '4'
>+++++                        a15 = '5'
>++++++                       a16 = '6'
>+++++++                      a17 = '7'
>++++++++                     a18 = '8'
>+++++++++                    a19 = '9'
>
++++++++++
++++++++++
++++++++++
++++++++++
++++++++++
++++++++++
++++++++++
++++++++++
++++++++++
++++++++++
++++++++++
++++++++++
++++                          a20 = '|'
>
++++++++++
++++++++++
++++++++++
++++++++++
+++                           a21 = 'plus'
>
++++++++++
++++++++++
++++++++++
++++++++++
+++++                         a22 = 'minus'
>
++++++++++                    a23 = '\n'
<<<
<<<<<<<<<<
<<<<<<<<<<

[                              GameLoop display
 >>>>>>>>>>>.                  a11
 >>>>>>>>>.                    a20
 <<<<<<<<.                     a12
 >>>>>>>>.                     a20
 <<<<<<<.                      a13
 >>>>>>>>>>.                   a23
 <.                            a22
 <.                            a21
 >.                            a22
 <.                            a21
 >.                            a22
 >.                            a23
 <<<<<<<<<.                    a14
 >>>>>>.                       a20
 <<<<<.                        a15
 >>>>>.                        a20
 <<<<.                         a16
 >>>>>>>.                      a23
 <.                            a22
 <.                            a21
 >.                            a22
 <.                            a21
 >.                            a22
 >.                            a23
 <<<<<<.                       a17
 >>>.                          a20
 <<.                           a18
 >>.                           a20
 <.                            a19
 >>>>.                         a23
 <<<<<<<<<<
 <<<<<<<<<<
 <<,                           read a1
 >,[-]>>                       ignore '\n'
#case1
 >>>>>>>
 [-
  <<<<<<<<
  +<+
  >>>>>>>>>
 ]                             a11 ) a3 = a2
 <<<<<<<<<
 [
  ->>>>>>>>>
  +<<<<<<<<<
 ]                             a2 ) a11
 >---------
 ----------
 ----------
 ----------
 ----------                    
                               a3 minus= '1'
 >+<[>-<[-]]                   a4 = a3 == 0
 >[[-]                         if(a4)
  <<<
  [->+>+<<]                     a1 ) a2 = a3
  >[-<+>]                       a2 ) a1
  >---------
  ----------
  ----------
  ----------
  ----------                    
                                a3 minus= '1'
  >+<[>-<[-]]                   a4 = a3 == 0
  >[[-]                         if(a4)
   >>>>>>[-<<+<+>>>]             a10 ) a8 = a7
   +<<<[>>>-<<<[-]]              a10 = a7 == 0
   >[[-]                         if(a8)
    >>>[-]                        a11 = 0
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++                      a11 = 'X'
    <<<
   ]                             endif(a8)
   >[-<+<+>>]                    a9 ) a8 = a7
   +<<[>>-<<[-]]                 a9 = a7 == 0
   >[[-]                         if(a8)
    >>>[-]                        a11 = 0
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    +++++++++                     a11 = 'O'
    <<<
   ]                             endif(a8)
   <<<+                          a5 = 1
   <
  ]                             endif(a4)
 ]                             endif(a4)
#case2
 >>>>>>>>
 [-
  <<<<<<<<<
  +<+
  >>>>>>>>>>
 ]                             a12 ) a3 = a2
 <<<<<<<<<<
 [
  ->>>>>>>>>>
  +<<<<<<<<<<
 ]                             a2 ) a12
 >---------
 ----------
 ----------
 ----------
 ----------                    
 -                             a3 minus= '2'
 >+<[>-<[-]]                   a4 = a3 == 0
 >[[-]                         if(a4)
  <<<
  [->+>+<<]                     a1 ) a2 = a3
  >[-<+>]                       a2 ) a1
  >---------
  ----------
  ----------
  ----------
  ----------                    
  -                             a3 minus= '2'
  >+<[>-<[-]]                   a4 = a3 == 0
  >[[-]                         if(a4)
   >>>>>>[-<<+<+>>>]             a10 ) a8 = a7
   +<<<[>>>-<<<[-]]              a10 = a7 == 0
   >[[-]                         if(a8)
    >>>>[-]                       a12 = 0
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++                      a12 = 'X'
    <<<<
   ]                             endif(a8)
   >[-<+<+>>]                    a9 ) a8 = a7
   +<<[>>-<<[-]]                 a9 = a7 == 0
   >[[-]                         if(a8)
    >>>>[-]                       a12 = 0
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    +++++++++                     a12 = 'O'
    <<<<
   ]                             endif(a8)
   <<<+                          a5 = 1
   <
  ]                             endif(a4)
 ]                             endif(a4)
#case3
 >>>>>>>>>
 [-
  <<<<<<<<<<
  +<+
  >>>>>>>>>>>
 ]                             a13 ) a3 = a2
 <<<<<<<<<<<
 [
  ->>>>>>>>>>>
  +<<<<<<<<<<<
 ]                             a2 ) a13
 >---------
 ----------
 ----------
 ----------
 ----------                    
 --                            a3 minus= '3'
 >+<[>-<[-]]                   a4 = a3 == 0
 >[[-]                         if(a4)
  <<<
  [->+>+<<]                     a1 ) a2 = a3
  >[-<+>]                       a2 ) a1
  >---------
  ----------
  ----------
  ----------
  ----------                    
  --                            a3 minus= '3'
  >+<[>-<[-]]                   a4 = a3 == 0
  >[[-]                         if(a4)
   >>>>>>[-<<+<+>>>]             a10 ) a8 = a7
   +<<<[>>>-<<<[-]]              a10 = a7 == 0
   >[[-]                         if(a8)
    >>>>>[-]                      a13 = 0
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++                      a13 = 'X'
    <<<<<
   ]                             endif(a8)
   >[-<+<+>>]                    a9 ) a8 = a7
   +<<[>>-<<[-]]                 a9 = a7 == 0
   >[[-]                         if(a8)
    >>>>>[-]                      a13 = 0
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    +++++++++                     a13 = 'O'
    <<<<<
   ]                             endif(a8)
   <<<+                          a5 = 1
   <
  ]                             endif(a4)
 ]                             endif(a4)
#case4
 >>>>>>>>>>
 [-
  <<<<<<<<<<<
  +<+
  >>>>>>>>>>>>
 ]                             a14 ) a3 = a2
 <<<<<<<<<<<<
 [
  ->>>>>>>>>>>>
  +<<<<<<<<<<<<
 ]                             a2 ) a14
 >---------
 ----------
 ----------
 ----------
 ----------                    
 ---                           a3 minus= '4'
 >+<[>-<[-]]                   a4 = a3 == 0
 >[[-]                         if(a4)
  <<<
  [->+>+<<]                     a1 ) a2 = a3
  >[-<+>]                       a2 ) a1
  >---------
  ----------
  ----------
  ----------
  ----------                    
  ---                           a3 minus= '4'
  >+<[>-<[-]]                   a4 = a3 == 0
  >[[-]                         if(a4)
   >>>>>>[-<<+<+>>>]             a10 ) a8 = a7
   +<<<[>>>-<<<[-]]              a10 = a7 == 0
   >[[-]                         if(a8)
    >>>>>>[-]                     a14 = 0
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++                      a14 = 'X'
    <<<<<<
   ]                             endif(a8)
   >[-<+<+>>]                    a9 ) a8 = a7
   +<<[>>-<<[-]]                 a9 = a7 == 0
   >[[-]                         if(a8)
    >>>>>>[-]                     a14 = 0
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    +++++++++                     a14 = 'O'
    <<<<<<
   ]                             endif(a8)
   <<<+                          a5 = 1
   <
  ]                             endif(a4)
 ]                             endif(a4)
#case5
 >>>>>>>>>>>
 [-
  <<<<<<<<<<<<
  +<+
  >>>>>>>>>>>>>
 ]                             a15 ) a3 = a2
 <<<<<<<<<<<<<
 [
  ->>>>>>>>>>>>>
  +<<<<<<<<<<<<<
 ]                             a2 ) a15
 >---------
 ----------
 ----------
 ----------
 ----------                    
 ----                          a3 minus= '5'
 >+<[>-<[-]]                   a4 = a3 == 0
 >[[-]                         if(a4)
  <<<
  [->+>+<<]                     a1 ) a2 = a3
  >[-<+>]                       a2 ) a1
  >---------
  ----------
  ----------
  ----------
  ----------                    
  ----                          a3 minus= '5'
  >+<[>-<[-]]                   a4 = a3 == 0
  >[[-]                         if(a4)
   >>>>>>[-<<+<+>>>]             a10 ) a8 = a7
   +<<<[>>>-<<<[-]]              a10 = a7 == 0
   >[[-]                         if(a8)
    >>>>>>>[-]                    a15 = 0
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++                      a15 = 'X'
    <<<<<<<
   ]                             endif(a8)
   >[-<+<+>>]                    a9 ) a8 = a7
   +<<[>>-<<[-]]                 a9 = a7 == 0
   >[[-]                         if(a8)
    >>>>>>>[-]                    a15 = 0
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    +++++++++                     a15 = 'O'
    <<<<<<<
   ]                             endif(a8)
   <<<+                          a5 = 1
   <
  ]                             endif(a4)
 ]                             endif(a4)
#case6
 >>>>>>>>>>>>
 [-
  <<<<<<<<<<<<<
  +<+
  >>>>>>>>>>>>>>
 ]                             a16 ) a3 = a2
 <<<<<<<<<<<<<<
 [
  ->>>>>>>>>>>>>>
  +<<<<<<<<<<<<<<
 ]                             a2 ) a16
 >---------
 ----------
 ----------
 ----------
 ----------                    
 -----                         a3 minus= '6'
 >+<[>-<[-]]                   a4 = a3 == 0
 >[[-]                         if(a4)
  <<<
  [->+>+<<]                     a1 ) a2 = a3
  >[-<+>]                       a2 ) a1
  >---------
  ----------
  ----------
  ----------
  ----------                    
  -----                         a3 minus= '6'
  >+<[>-<[-]]                   a4 = a3 == 0
  >[[-]                         if(a4)
   >>>>>>[-<<+<+>>>]             a10 ) a8 = a7
   +<<<[>>>-<<<[-]]              a10 = a7 == 0
   >[[-]                         if(a8)
    >>>>>>>>[-]                   a16 = 0
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++                      a16 = 'X'
    <<<<<<<<
   ]                             endif(a8)
   >[-<+<+>>]                    a9 ) a8 = a7
   +<<[>>-<<[-]]                 a9 = a7 == 0
   >[[-]                         if(a8)
    >>>>>>>>[-]                   a16 = 0
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    +++++++++                     a16 = 'O'
    <<<<<<<<
   ]                             endif(a8)
   <<<+                          a5 = 1
   <
  ]                             endif(a4)
 ]                             endif(a4)
#case7
 >>>>>>>>>>>>>
 [-
  <<<<<<<<<<<<<<
  +<+
  >>>>>>>>>>>>>>>
 ]                             a17 ) a3 = a2
 <<<<<<<<<<<<<<<
 [
  ->>>>>>>>>>>>>>>
  +<<<<<<<<<<<<<<<
 ]                             a2 ) a17
 >---------
 ----------
 ----------
 ----------
 ----------                    
 ------                        a3 minus= '7'
 >+<[>-<[-]]                   a4 = a3 == 0
 >[[-]                         if(a4)
  <<<
  [->+>+<<]                     a1 ) a2 = a3
  >[-<+>]                       a2 ) a1
  >---------
  ----------
  ----------
  ----------
  ----------                    
  ------                        a3 minus= '7'
  >+<[>-<[-]]                   a4 = a3 == 0
  >[[-]                         if(a4)
   >>>>>>[-<<+<+>>>]             a10 ) a8 = a7
   +<<<[>>>-<<<[-]]              a10 = a7 == 0
   >[[-]                         if(a8)
    >>>>>>>>>[-]                  a17 = 0
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++                      a14 = 'X'
    <<<<<<<<<
   ]                             endif(a8)
   >[-<+<+>>]                    a9 ) a8 = a7
   +<<[>>-<<[-]]                 a9 = a7 == 0
   >[[-]                         if(a8)
    >>>>>>>>>[-]                  a17 = 0
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    +++++++++                     a17 = 'O'
    <<<<<<<<<
   ]                             endif(a8)
   <<<+                          a5 = 1
   <
  ]                             endif(a4)
 ]                             endif(a4)
#case8
 >>>>>>>>>>>>>>
 [-
  <<<<<<<<<<<<<<<
  +<+
  >>>>>>>>>>>>>>>>
 ]                             a18 ) a3 = a2
 <<<<<<<<<<<<<<<<
 [
  ->>>>>>>>>>>>>>>>
  +<<<<<<<<<<<<<<<<
 ]                             a2 ) a18
 >---------
 ----------
 ----------
 ----------
 ----------                    
 -------                       a3 minus= '8'
 >+<[>-<[-]]                   a4 = a3 == 0
 >[[-]                         if(a4)
  <<<
  [->+>+<<]                     a1 ) a2 = a3
  >[-<+>]                       a2 ) a1
  >---------
  ----------
  ----------
  ----------
  ----------                    
  -------                       a3 minus= '8'
  >+<[>-<[-]]                   a4 = a3 == 0
  >[[-]                         if(a4)
   >>>>>>[-<<+<+>>>]             a10 ) a8 = a7
   +<<<[>>>-<<<[-]]              a10 = a7 == 0
   >[[-]                         if(a8)
    >>>>>>>>>>[-]                 a18 = 0
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++                      a18 = 'X'
    <<<<<<<<<<
   ]                             endif(a8)
   >[-<+<+>>]                    a9 ) a8 = a7
   +<<[>>-<<[-]]                 a9 = a7 == 0
   >[[-]                         if(a8)
    >>>>>>>>>>[-]                 a18 = 0
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    +++++++++                     a18 = 'O'
    <<<<<<<<<<
   ]                             endif(a8)
   <<<+                          a5 = 1
   <
  ]                             endif(a4)
 ]                             endif(a4)
#case9
 >>>>>>>>>>>>>>>
 [-
  <<<<<<<<<<<<<<<<
  +<+
  >>>>>>>>>>>>>>>>>
 ]                             a19 ) a3 = a2
 <<<<<<<<<<<<<<<<<
 [
  ->>>>>>>>>>>>>>>>>
  +<<<<<<<<<<<<<<<<<
 ]                             a2 ) a19
 >---------
 ----------
 ----------
 ----------
 ----------                    
 --------                      a3 minus= '9'
 >+<[>-<[-]]                   a4 = a3 == 0
 >[[-]                         if(a4)
  <<<
  [->+>+<<]                     a1 ) a2 = a3
  >[-<+>]                       a2 ) a1
  >---------
  ----------
  ----------
  ----------
  ----------                    
  --------                      a3 minus= '9'
  >+<[>-<[-]]                   a4 = a3 == 0
  >[[-]                         if(a4)
   >>>>>>[-<<+<+>>>]             a10 ) a8 = a7
   +<<<[>>>-<<<[-]]              a10 = a7 == 0
   >[[-]                         if(a8)
    >>>>>>>>>>>[-]                a19 = 0
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++                      a19 = 'X'
    <<<<<<<<<<<
   ]                             endif(a8)
   >[-<+<+>>]                    a9 ) a8 = a7
   +<<[>>-<<[-]]                 a9 = a7 == 0
   >[[-]                         if(a8)
    >>>>>>>>>>>[-]                a19 = 0
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    ++++++++++
    +++++++++                     a19 = 'O'
    <<<<<<<<<<<
   ]                             endif(a8)
   <<<+                          a5 = 1
   <
  ]                             endif(a4)
 ]                             endif(a4)
 >[[-]                         if(a5)
  >>>>>>>>>>
  >>>>>>>>>+                    inc a24 total moves
  [->+>+<<]                     a24 ) a25 = a26
  >[-<+>]                       a25 ) a24
  +                             a25 = 1
  >---------                    a26 = a26 != 9
  [[-]                          if(a26)
   <->                           a25 = 0
  ]                             endif(a26)
  <[[-]                         if(a25)
   <<<<<<<<<<
   <<<<<<<<<<
   <<<+                          a2 = 1 Tie
   <<[-]                         a0 = 0 GameOver
   >>>>>>>>>>
   >>>>>>>>>>
   >>>>>
  ]                             endif(a25)
  <<<<<<<<<<
  <<<<<<<<<<
 ]                             endif(a5)
#check123
 >>>>>>
 [-<<<+<<<+>>>>>>]             a11 ) a8 = a5
 <<<
 [->>>+<<<]                    a8 ) a11
 >>>>
 [-<<<<+<<+>>>>>>]             a12 ) a8 = a6
 <<<<
 [->>>>+<<<<]                  a8 ) a12
 >>>>>
 [-<<<<<+<+>>>>>>]             a13 ) a8 = a7
 <<<<<
 [->>>>>+<<<<<]                a8 ) a13
 <
 [-<-<->>]                     a7 = a6 = a5 minus a7
 <[[-]<<+>>]                   a4 = a6 != a7
 <[[-]<+>]                     a4 = a4 || a5 != a7
 <<+>[[-]<->]                  a3 = !a4
 <[[-]                         if(a3)
  <[-]                          a2 = 0 noTie
  <<[-]>>>                      a0 = 0 GameOver
 ]                             endif(a3)
#check456
 >>>>>>>>>>>
 [-<<<<<<+<<<+>>>>>>>>>]       a14 ) a8 = a5
 <<<<<<
 [->>>>>>+<<<<<<]              a8 ) a14
 >>>>>>>
 [-<<<<<<<+<<+>>>>>>>>>]       a15 ) a8 = a6
 <<<<<<<
 [->>>>>>>+<<<<<<<]            a8 ) a15
 >>>>>>>>
 [-<<<<<<<<+<+>>>>>>>>>]       a16 ) a8 = a7
 <<<<<<<<
 [->>>>>>>>+<<<<<<<<]          a8 ) a16
 <
 [-<-<->>]                     a7 = a6 = a5 minus a7
 <[[-]<<+>>]                   a4 = a6 != a7
 <[[-]<+>]                     a4 = a4 || a5 != a7
 <<+>[[-]<->]                  a3 = !a4
 <[[-]                         if(a3)
  <[-]                          a2 = 0 noTie
  <<[-]>>>                      a0 = 0 GameOver
 ]                             endif(a3)
#check789
 >>>>>>>>>>>>>>
 [-<<<<<<<<<+<<<+>>>>>>>>>>>>] a17 ) a8 = a5
 <<<<<<<<<
 [->>>>>>>>>+<<<<<<<<<]        a8 ) a17
 >>>>>>>>>>
 [-<<<<<<<<<<+<<+>>>>>>>>>>>>] a18 ) a8 = a6
 <<<<<<<<<<
 [->>>>>>>>>>+<<<<<<<<<<]      a8 ) a18
 >>>>>>>>>>>
 [-<<<<<<<<<<<+<+>>>>>>>>>>>>] a19 ) a8 = a7
 <<<<<<<<<<<
 [->>>>>>>>>>>+<<<<<<<<<<<]    a8 ) a19
 <
 [-<-<->>]                     a7 = a6 = a5 minus a7
 <[[-]<<+>>]                   a4 = a6 != a7
 <[[-]<+>]                     a4 = a4 || a5 != a7
 <<+>[[-]<->]                  a3 = !a4
 <[[-]                         if(a3)
  <[-]                          a2 = 0 noTie
  <<[-]>>>                      a0 = 0 GameOver
 ]                             endif(a3)
#check147
 >>>>>>>>
 [-<<<+<<<+>>>>>>]             a11 ) a8 = a5
 <<<
 [->>>+<<<]                    a8 ) a11
 >>>>>>
 [-<<<<<<+<<+>>>>>>>>]         a14 ) a8 = a6
 <<<<<<
 [->>>>>>+<<<<<<]              a8 ) a14
 >>>>>>>>>
 [-<<<<<<<<<+<+>>>>>>>>>>]     a17 ) a8 = a7
 <<<<<<<<<
 [->>>>>>>>>+<<<<<<<<<]        a8 ) a17
 <
 [-<-<->>]                     a7 = a6 = a5 minus a7
 <[[-]<<+>>]                   a4 = a6 != a7
 <[[-]<+>]                     a4 = a4 || a5 != a7
 <<+>[[-]<->]                  a3 = !a4
 <[[-]                         if(a3)
  <[-]                          a2 = 0 noTie
  <<[-]>>>                      a0 = 0 GameOver
 ]                             endif(a3)
#check258
 >>>>>>>>>
 [-<<<<+<<<+>>>>>>>]           a12 ) a8 = a5
 <<<<
 [->>>>+<<<<]                  a8 ) a12
 >>>>>>>
 [-<<<<<<<+<<+>>>>>>>>>]       a15 ) a8 = a6
 <<<<<<<
 [->>>>>>>+<<<<<<<]            a8 ) a15
 >>>>>>>>>>
 [-<<<<<<<<<<+<+>>>>>>>>>>>]   a18 ) a8 = a7
 <<<<<<<<<<
 [->>>>>>>>>>+<<<<<<<<<<]      a8 ) a18
 <
 [-<-<->>]                     a7 = a6 = a5 minus a7
 <[[-]<<+>>]                   a4 = a6 != a7
 <[[-]<+>]                     a4 = a4 || a5 != a7
 <<+>[[-]<->]                  a3 = !a4
 <[[-]                         if(a3)
  <[-]                          a2 = 0 noTie
  <<[-]>>>                      a0 = 0 GameOver
 ]                             endif(a3)
#check369
 >>>>>>>>>>
 [-<<<<<+<<<+>>>>>>>>]         a13 ) a8 = a5
 <<<<<
 [->>>>>+<<<<<]                a8 ) a13
 >>>>>>>>
 [-<<<<<<<<+<<+>>>>>>>>>>]     a16 ) a8 = a6
 <<<<<<<<
 [->>>>>>>>+<<<<<<<<]          a8 ) a16
 >>>>>>>>>>>
 [-<<<<<<<<<<<+<+>>>>>>>>>>>>] a19 ) a8 = a7
 <<<<<<<<<<<
 [->>>>>>>>>>>+<<<<<<<<<<<]    a8 ) a19
 <
 [-<-<->>]                     a7 = a6 = a5 minus a7
 <[[-]<<+>>]                   a4 = a6 != a7
 <[[-]<+>]                     a4 = a4 || a5 != a7
 <<+>[[-]<->]                  a3 = !a4
 <[[-]                         if(a3)
  <[-]                          a2 = 0 noTie
  <<[-]>>>                      a0 = 0 GameOver
 ]                             endif(a3)
#check159
 >>>>>>>>
 [-<<<+<<<+>>>>>>]             a11 ) a8 = a5
 <<<
 [->>>+<<<]                    a8 ) a11
 >>>>>>>
 [-<<<<<<<+<<+>>>>>>>>>]       a15 ) a8 = a6
 <<<<<<<
 [->>>>>>>+<<<<<<<]            a8 ) a15
 >>>>>>>>>>>
 [-<<<<<<<<<<<+<+>>>>>>>>>>>>] a19 ) a8 = a7
 <<<<<<<<<<<
 [->>>>>>>>>>>+<<<<<<<<<<<]    a8 ) a19
 <
 [-<-<->>]                     a7 = a6 = a5 minus a7
 <[[-]<<+>>]                   a4 = a6 != a7
 <[[-]<+>]                     a4 = a4 || a5 != a7
 <<+>[[-]<->]                  a3 = !a4
 <[[-]                         if(a3)
  <[-]                          a2 = 0 noTie
  <<[-]>>>                      a0 = 0 GameOver
 ]                             endif(a3)
#check357
 >>>>>>>>>>
 [-<<<<<+<<<+>>>>>>>>]         a13 ) a8 = a5
 <<<<<
 [->>>>>+<<<<<]                a8 ) a13
 >>>>>>>
 [-<<<<<<<+<<+>>>>>>>>>]       a15 ) a8 = a6
 <<<<<<<
 [->>>>>>>+<<<<<<<]            a8 ) a15
 >>>>>>>>>
 [-<<<<<<<<<+<+>>>>>>>>>>]     a17 ) a8 = a7
 <<<<<<<<<
 [->>>>>>>>>+<<<<<<<<<]        a8 ) a17
 <
 [-<-<->>]                     a7 = a6 = a5 minus a7
 <[[-]<<+>>]                   a4 = a6 != a7
 <[[-]<+>]                     a4 = a4 || a5 != a7
 <<+>[[-]<->]                  a3 = !a4
 <[[-]                         if(a3)
  <[-]                          a2 = 0 noTie
  <<[-]>>>                      a0 = 0 GameOver
 ]                             endif(a3)

 <<<
]                             endGameLoop
>>>>>>>>>>>.                  a11
>>>>>>>>>.                    a20
<<<<<<<<.                     a12
>>>>>>>>.                     a20
<<<<<<<.                      a13
>>>>>>>>>>.                   a23
<.                            a22
<.                            a21
>.                            a22
<.                            a21
>.                            a22
>.                            a23
<<<<<<<<<.                    a14
>>>>>>.                       a20
<<<<<.                        a15
>>>>>.                        a20
<<<<.                         a16
>>>>>>>.                      a23
<.                            a22
<.                            a21
>.                            a22
<.                            a21
>.                            a22
>.                            a23
<<<<<<.                       a17
>>>.                          a20
<<.                           a18
>>.                           a20
<.                            a19
>>>>.                         a23
<<<<<<<<<<
<<<<<<<<<<
[-]+                          a3 = 1
<[[-]                         if(a2)
 >
 ++++++++++
 ++++++++++
 ++++++++++
 ++++++++++
 ++++++++++
 ++++++++++
 ++++++++++
 ++++++++++
 +++.                          'T'
 ----------
 -.                            'I'
 ----.                         'E'
 [-]                           a3 = 0
 <
]                             endif(a2)
>[[-]                         if(a3)
 >>
 [-]                           a5 = 0
 ++++++++++
 ++++++++++
 ++++++++++
 ++++++++++
 ++++++++++
 ++++++++++
 ++++++++++
 ++++++++                      a5 = 'N'
 >[-]                          a6 = 0
 ++++++++++
 ++++++++++
 ++++++++++
 ++++++++++
 ++++++++++
 ++++++++++
 ++++++++++
 +++++++++                     a6 = 'O'
 >[-]                          a7 = 0
 ++++++++++
 ++++++++++
 ++++++++++
 ++++++++++
 ++++++++++
 ++++++++++
 ++++++++++
 ++++++++++
 +++++++                       a7 = 'W'
 >[-]                          a8 = 0
 ++++++++++
 ++++++++++
 ++++++++++
 ++                            a8 = ' '
 >
 [[-]                          if(a9)
  ++++++++++
  ++++++++++
  ++++++++++
  ++++++++++
  ++++++++++
  ++++++++++
  ++++++++++
  ++++++++++
  ++++++++                      a9 = 'X'
  .[-]                          'X'
 ]                             endif(a9)
 >[[-]                         if(a10)
  <<<<.>>>>                     'O'
 ]                             endif(a10)
 <<.<.<.<.                     " WON"
 <<
]                              endif(a3)
>>>>>>>>>>
>>>>>>>>>>
.                             a23