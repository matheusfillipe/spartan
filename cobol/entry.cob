       identification division.
       program-id. entry.
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
       01 loutput PIC A(512) value NULLS.
       procedure division using lin, loutput.
           display "COBOL: """function trim(lin trailing)""""
           display " "
      *    move lin to loutput
           exit program.

