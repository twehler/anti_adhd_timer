required packages: xsct, wmcrtl (does NOT work for Wayland yet!)


SUMMARY:

This application combines two functionalities: Holding you accountable to a self-set schedule and fixing your bedtime in a healthy way, with you being in 100% control over everything! It also divides the time spaces into fully-fledged Pomodoro-blocks. At the end of every Pomodoro session, it openes a "log-file", where you can specify what you did during the last Pomodoro-session. This file won't disappear from the top of the screen until you close it.

Despite the rigorous structure, this application will make your life very stress-free compared to before, since you can now empty your head and purely focus on your activities!


HOW TO USE:

1. Specify your plans for the day inside a .txt file in the following format:
  
    07:00 - 08:00 - Task 1 (here you can write any character you want)
    
    08:15 - 10:00 - Task 2
    
    10:00 - 15:35 - Task 3
    
    etc...

    The timer will throw notifications and let the screen blink for a short time when Pomodoro-sessions or time-intervals for tasks end in order to alert you.


2. Execute the binary file inside the terminal, pass the txt-file as an argument. After executing, you will be asked when you want to go to bed. The program will start after the input and step-by-step take control over the screen color temperature, starting with 3 hours before bedtime. This is being handled more intensely than normal blue-light filters do, with scientifically proven kelvin-colors, which will make you sleep far beter than before. When bedtime is reached, a temperature of 1000K is being applied, making you want to go to bed.
