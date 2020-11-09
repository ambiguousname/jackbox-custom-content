# What is this?

It's a program for adding your own content to the Jackbox Party Pack 7. Check the releases tab for the latest features.

# Install instructions

- Unzip the contents of the .ZIP file into the "games" directory of your Jackbox Party Pack 7 install folder. (For steam on Windows: C:\Program Files (x86)\Steam\steamapps\common\The Jackbox Party Pack 7\games)
- Run "Jackbox Party Pack Custom.exe".

## HELP, EVERYTHING IS BROKEN

If you're using Steam, go to the Jackbox Party Pack 7 in your Steam Library. Right click on the game's icon or name, click "Properties".
In the popup window, click on "Local Files". Then click "Verify integrity of game files..." That should fix everything.

## EVERYTHING'S STILL BROKEN

Uninstall and reinstall the Jackbox Party Pack 7

## NOPE, IT STILL DOESN'T WORK

Delete everything from your Jackbox Party Pack 7 folder, then find the "Verify integrity of game files..." button and click it.


# Making custom responses to specific text for Quiplash 3:

NOTE: This only works for Round 1 and 2 questions. You can't have custom responses for Final Round questions. 

Okay, I'm note entirely sure how this works, but say you have a prompt like:

`Oh no, my dog ate my <BLANK>!`

And you want a specific response if someone says "homework".

In the "What to filter field", you'd put:

`<PRONOUN> Homework|<PROUNOUN> homework|hw|<PRONOUN> hw`

You should already know that Jackbox uses tags like <ANYPLAYER> and <BLANK> for questions, and so for their responses to specific answers,
they use the tags <ARTICLE> (Like "the", "a", "an") and <PRONOUN> (Like "I", "My", "His", "Hers"). Jackbox will also separate possible answers by
a "|" sign.

You should also try to anticipate alternate answers, like abbreviations or misspellings. Let's look at another example.

For the question "What skin tags probably taste like", Jackbox has:

`<ARTICLE>  chicken | chicken|<ARTICLE>  chiken | chiken|<ARTICLE> chikin | chikin`

as a filter, trying to detect things like "A chicken", "chicken", "A chiken", "chiken", etc.

Note the spaces in between the "|" signs in the example above. It's just a formatting thing, I don't think it really matters. You can add spaces only if you want to.

Once you've added the filtering, then you can add your response audio as a .ogg file. Jackbox also requires a transcript of your response for captioning purposes (I think),
so you should write out what you've said in the "Transcript of your response: " field.