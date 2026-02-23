SUMMARY:

This application combines two functionalities: Holding you accountable for your tasks and radically fixing your bed-routine. This is being done with changing screen colors and notifications.
 


HOW TO USE:

1. Specify your plans for the day inside a .txt file in the following format:
  
    07:00 - 08:00 - Task1
    
    08:15 - 10:00 - Task2
    
    10:00 - 15:35 - Task3
    
    etc...

    The timer will throw notifications and let the screen blink for a short time in order to alert the user.
    It will also divide the task times in Pomodoro-intervals as best as possible (currently somethat bugged , but still usable)

2. Execute the binary file inside the terminal, specify your bedtime and give the location of the .txt file. The timer will
   also take care of your bedtime, by setting reminders and decreasing the color-temperature of the screen with xsct step by step.
   If bedtime has arrived, the screen will become monochrome-red, rendering it half-useless (you can still use it, but it will be unpleasant)
