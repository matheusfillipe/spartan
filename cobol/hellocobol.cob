       identification division.
       program-id. hellocobol.
       author. matheus.
       date-written. December 7th 2021
       environment division.
       configuration section.
       data division.
       file section.
       working-storage section.
       01 ws-comp PIC 9(4)V99.
       linkage section.
       01 lin PIC A(512).
       01 lsin PIC 99.
       01 loutput PIC A(512) value NULLS.
       procedure division using lin, lsin, loutput.
           display " "
           display "COBOL START"
           display "Length: " lsin
           compute ws-comp = lsin/10.
           display "Divided by 10: " ws-comp
           display "String: " lin(1:lsin)
      *    display "Say something: " with no advancing
      *    accept loutput

           move "Hello from cobol" to loutput
           display "COBOL END"
           exit program.

