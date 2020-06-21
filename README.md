# work
Is a simple command line tool for tracking the daily activities. The activities are writen as simple lines in an ordinary  text file.

E.g.$ nano monday-tasks.txt

08:00 08:15 Fixing JavaScript in UI Code
08:15 12:00 Refactoring of HTML
13:00 15:00 Meeting Next Generation Shopping Cart
15:00 17:30 UX debriefing we George

Each line has a start and end time, followed by a description of the activity. To calculate the work of the whole day, simple issue following command in the console.

$ work monday-tasks.txt
```
--------------------------------------------------------------
Simple Report
--------------------------------------------------------------
0.25    Fixing JavaScript in UI Code
3.75    Refactoring of HTML
2.00    Meeting Next Generation Shopping Cart
2.50    UX debriefing we George
0.25    Refactoring of HTML
--------------------------------------------------------------

Summarized
--------------------------------------------------------------
0.25    Fixing JavaScript in UI Code
4.00    Refactoring of HTML
2.50    UX debriefing we George
2.00    Meeting Next Generation Shopping Cart
--------------------------------------------------------------
Total:  8.75
--------------------------------------------------------------
```
