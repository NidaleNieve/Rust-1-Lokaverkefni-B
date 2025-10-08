# Fyrsta promptið Claude 4.5 Sonnet
Create this project by following ALL of these instructions, you have to consider them ALL, and dont skip a SINGLE point. You have to remember to follow all points and instructions. 
Here are some additional notes that you HAVE to keep in mind, this is stuff other people forgot or got wrong, and you HAVE to get it right:

I want you to edit this project so the gui for "Uppfæra staðsetningu" uses the same gui as when you are registering a new item. With a drop-down menu and sliders. 

Additionally I want you to drastically improve the printing experience. With the output not being at the top and grey and barely visible. More importantly, you can't just print it directly, there has to be some formatting and prettifying, like: "Skjávarpi með ID: 2, kostar 345kr, með 1234 lumen og er staðsettur í HA-354" instead of this: "Skjávarpi(id=2, 1234 lm, 345 kr., HA-354)" which is horrid.

Also, the printing options are not as expected. These are the supposed options: "það þarf að vera hægt að prenta á skjá lista með öllum búnaði.
það þarf að vera hægt að prenta á skjá lista með öllum búnaði í ákveðnu húsi.
Prenta á skjá búnað af ákveðinni gerð, t.d. alla stóla.
Prenta á skjá allan búnað í:
    ákveðinni stofu.
    ákveðinni hæð í ákveðnu húsi.
"
But instead its only "Birta búnað í ákveðinni stofu", and "Birta búnað á ákveðinni hæð í húsi". 
We should have a printing section with togglable options, instead of five different sections

Also, the max number for Stofa should be 99.
Also make it so that you can click on the columns in the list to reorder the list based on that column, ascending and descending. And then a button to reset (only becomes visible after changing the order).
Make sure you're using Display and TryFrom for everything and "Hafðu hvert struct og enum í sér skrá."
Also Instead of having many different sections and elements in the UI, make them concise and have different actions togglable (instead of those actions having their own seperate section).
Make sure there is a WOW factor to show the teacher.
Also one final thing, when the id is more than one digit, it breaks visually, putting one number on top of the other one.

## GPT-5-Mini og Claude 4.5 Sonnet Chat, til þess að laga og betrumbæta appið
Prompt 1:
Lets add a color scheme, light blue and modern. Hovering over items makes them react and clicking them also.

I want you to change the "Birta menu" to be "Prenta" and remove the search to its own tab. Then also merge the uppfæra and eyða tabs to just be "Breyta" where you can select an item by its ID (it shows the items info like in the search menu after inputting the id), like it is now and then you can select whether or not to delete or change it

Prompt 2:
Go over the project and verify whether or not EVERY single struct and stuff that should use Display and TryFrom uses it. ALL Structs should have it, and use it for everything, instead of manually printing and using new::

Prompt 3:
Continue: "Continue to iterate?"

Prompt 4:
Lets change the icon of the app to a 1200x1200 jpg image. Both inside the app and the apps icon itself. Both windows and mac

Prompt 5:
Read the error

Prompt 6:
YOU FIX IT

Prompt 7:
I want to fix the Export and Import from json so that it shows a file picker to save and pick files. Also i want to add a button in the right side of the screen that shows the full list on the right side in a side bar. Exactly like when you you use Prenta -> Allur Búnaður, with all its features, such as being able to reorder it etc. Hide the Lýsing initially unless the sidebar is resized. Finally, change most of the text to have more contrast and most importantly change the main header to be white "Búnaðaristi Tækniskólans". Then add my name at the bottom, it has to be persistant, like a footer "Daníel Snær Rodríguez, 2025

Prompt 8:
Ok the sidebar needs to be renamed to "sjá lista" and it needs to be more dynamic, allowing for more customization. So you can adjust the column width and you can make the entire thing as wide as you want with granular controls, you know? It cant jsut snap to Lýsing. Remember to use THE SAME code as you do for prenta, and add the endurstilla röðun button. Most importantly, the list doesnt update in realtime. I create a new item and it doesnt get added. Also fix the arrows in the column names, so it shows only the up arrow or down arrow if you are using that column for sorting, and depending whether it is descending or ascending.

One bug you made is that there are 2 different "nav bars" one is blue (which is ugly). and the second one is the old one with the badly legible text. Remove the old one, and change the colors of the new one to be more sublte

Prompt 9:
Add a bit of margin at the top so that the header text isnt at the very top and clipping. but the SAME issue with the snapping is happening to the sidebar. it ISN'T allowing me to resize however i want, even when i want it to clip into a column it should allow me to. Also the arrows aren't rendering they are bugged and only show a not found square, for both the sidebar and print menu

Prompt 10:
Add a bit of left margin to the header logo. Yet again the sidebar snaps when resizing its width. This is so annoying. You also have to keep in mind that the default behaviour should be that the width of the sidebar is only big enough to show the first four columns and not Lýsing. You can then extend the width however much you want to see lýsing. You also need to fix the arrows in the "prenta" menu. Additionally you need to make it so that importing a json file resets everything. It doesnt add them on top of the already existing list, it just replaces that list

Prompt 11:
When importing a JSON it gets sucesfully reset except the IDs, they dont reset based on the file, they get added on top of the current ones. Make sure the default sorting works after the import. Add a bit of top margin to the header text AND credit at the bottom with my name, remember to do both. Also add the new working arrows to the "prenta" columns. Also lets make it so that the search can search any column. And if there are multiple options you can select between them. Also you don't have to click enter to search, it just happens as you write, and if there is only one result then you dont have to select it


## GPT-5 Chat eftirá, fínpúss og auka fítusar
Prompt 1:
Make it so that when you search you can also sort by the columns like you can do in the sidebar, with the refresh button and arrows.
Addionally add the same arrows and arrow logic with them only appearing when you actually filter (and based on ascending/descending) to the columns in the "Prenta" menu.
Also make it so that you can click anywhere on an item to select it in the search menu, and in the process fix the fact that selecting doesnt work (Should take you to deleting and updating. Should also have a back button to take you back to main search).
Also to start the search you have to click the "sækja" button, which is stupid. It should search automatically from the start
Also fix the formatting and margins of both the header and footer (my name).

Prompt 2:
Make it so that you can click anywhere on the column in the search menu, even blank space, not just the text.
Also make the search list and Prenta list refresh automatically like the sidebar does.
Fix the footer being completely white and unreadable

Prompt 3:
Make it so that when you search you can also sort by the columns like you can do in the sidebar, with the refresh button and arrows.
Addionally add the same arrows and arrow logic with them only appearing when you actually filter (and based on ascending/descending) to the columns in the "Prenta" menu.
Also make it so that you can click anywhere on an item to select it in the search menu, and in the process fix the fact that selecting doesnt work (Should take you to deleting and updating. Should also have a back button to take you back to main search).
Also to start the search you have to click the "sækja" button, which is stupid. It should search automatically from the start
Also fix the formatting and margins of both the header and footer (my name).

Prompt 4:
Make the sidebar list and "prenta" menu list clickable like the EXACTLY search list. It would take you to the breyta tab like the search. With a back to prenta button if you cam from there an no button if clicked from the sidebar

Prompt 5:
Continue: "Continue to iterate?"

Prompt 6:
Remove the refresh buttons from search and "prenta" as the lists refresh automatically

Add a print function to the "Prenta" menu. So that you can print the current list (the one being displayed, which is dependant on the selected filters from that screen), it would bring up the systems print menu.
Also add a export to PDF option.

Prompt 7:
Make the list refresh faster, and fix the fact that when opening the app the search list doesnt load until you either open the sidebar or open the prenta menu
When exporting to pdf it doesnt save to the correct folder, like the save to json folder does. It just opens a browser window like the print button does and doesnt save the file

Prompt 8:
How do I compile the app into an exe and .app for mac, and rename it to Bunadarlisti, and be made by me. And then finally put them in releases in github

Prompt 9:
Add my name Daníel Snær Rodríguez and email danielsnaerr@gmail.com

Prompt 10:
Compiling objc-foundation v0.1.1
Compiling CLAUDE-Lokaverkefni-B v0.1.0 (/Users/nidale/Documents/Rust/CLAUDE-2-Lokaverkefni-B/Rust-1-Lokaverkefni-B)
...
fix this warning.
Change all the selection buttons (cicle, you can only pick one) in Prenta and in Skrá so that the inside circle is dark, black. ONLY change those buttons, NOT ANY of the text, only chang the circle so that it is more visible, and NO TEXT OR OTHER ELEMENT.
Additionally remove the birta button from the Prenta menu, put the. Vista í/úr JSON buttons be side by side, exactly like the prenta lista buttons.
Addionally, in the Breyta menu, make it autosearch, so you dont have to press the sækja button. You can just start typing in the id, add a small delay (300ms)

Prompt 11:
You also made the text besides the radio buttons black, and you cant read them. ONLY the buttons themselves have to be black, with the text besides them ligher like all other text. I also want you to add just a bit more margin between the buttons, right now it feels to close together with the text an everything.
Make the column headers (ID, Tegund etc.) always be centred. Make sure to do that for leita, prenta and the sidebar, all the same, and make sure to make the columns centered for ALL THREE OF THEM.

Prompt 12:
Now the radio buttons are entirely black, and the background is gray, which results in low visibility!! Fix that somehow, make the radio buttons clearly visible. You have to be able to easily tell which one is selected. Remove any dead code warning

Prompt 13:
Ok, this is awesome. Now we have to add a WOW factor. Some cool features that will make the teacher impressed. I'd also like to make ALL elements more reactive when you hover over them, so like a little animation plays, like a cool and good css javascript website.
In conclusion, plan out a couple of cool wow factor features, and then implement the reactive elements for EVERYTHING. Keep track of what you're doing with a to do list.

Prompt 14:
Continue: "Continue to iterate?"

Prompt 15:
I think the command palett thing is a good idea. Lets expand upon it. After you press command/ctrl + k the input field needs to already be highlighted and ready to type on. Then we need to add more options, and be able to perform actions right inside it. for example "eyda 23" deletes item 23.
"skra s-569" would fill those fields out in the skra menu (skólvörðuholt and 5 hæð and stofa 69). those are just some examples, expand upon them and it needs to be natural and feel intuitive.
Escape would also close the menu, also add a little x.

Prompt 16:
Fix the errors
Now lets add a tutorial menu, that goes through the menus and teaches you how the app works, and teaches you about all the features with arrows and good descriptions.

Prompt 17:
Lets remove the tutorial menu completely.
Lets add more options to the control panel, and more flexibility. Think about more options more ways to type out the same stuff for more flexibility and additonal actions. Think inside the box and outside it.
If you misspell by a few characters it still goes to the next most likely thing. Also make the tab key go to the next item in the seletion and enter to confirm that

Prompt 18:
Continue: "Continue to iterate?"

Prompt 19:
You didn't do what I told you to do. Be able to make typos and still it works, and more flexible commands, not just to navigate around the app but to interact with it and use it well and fast.