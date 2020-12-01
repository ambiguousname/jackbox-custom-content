# What is this?

It's a program for adding your own content (stuff like prompts, certain sound files, etc.) to the Jackbox Party Pack 7.

WARNING: I've really only tested this on Windows, it might not work on other operating systems. 

You may also want to keep a backup of the Jackbox Party Pack 7's files if you're not using Steam,
otherwise you're going to have to uninstall and reinstall your entire game if something goes wrong.

## Some screenshots of custom content in the Jackbox Party Pack 7:

![Talking Points](https://raw.githubusercontent.com/ambiguousname/jackbox-custom-content/main/screenshots/TalkingPoints.PNG)\
![Quiplash3](https://raw.githubusercontent.com/ambiguousname/jackbox-custom-content/main/screenshots/Quiplash3.PNG)

# Install instructions

## If you don't have python installed and are running on Windows:
- Go to the releases page and download the .ZIP file: https://github.com/ambiguousname/jackbox-custom-content/releases
- Unzip the contents of the .ZIP file into the "games" directory of your Jackbox Party Pack 7 install folder. (For steam on Windows: C:\Program Files (x86)\Steam\steamapps\common\The Jackbox Party Pack 7\games)
- Run "Jackbox Party Pack Custom.exe".

## If you do have python installed:
- Clone this repository
- Move "jppc.py" to the "games" directory of your Jackbox Party Pack 7 install folder. (For steam on Windows: C:\Program Files (x86)\Steam\steamapps\common\The Jackbox Party Pack 7\games)
- Run jppc.py in the terminal of your choice.

# Potential Questions/Problems

## Doesn't Quiplash 3 already have a way for you to make your own questions?

Yes, but there are a few differences. For one: making your own "episodes" in Quiplash 3 means you have to manually select them. If you add custom questions using this program, they'll get inserted into the normal rotation of questions. For two: using this program will allow you to add more stuff not available in Quiplash 3's "episodes" feature, like audio files to read the questions and custom responses to certain answers. And this program can be used for more games than Quiplash 3.

## HELP, EVERYTHING IS BROKEN AND/OR I REMOVED ALL NON-CUSTOM CONTENT AND CAN'T GET IT BACK

If you're using Steam, go to the Jackbox Party Pack 7 in your Steam Library. Right click on the game's icon or name, click "Properties".
In the popup window, click on "Local Files". Then click "Verify integrity of game files..." That should fix everything.

## EVERYTHING'S STILL BROKEN

Uninstall and reinstall the Jackbox Party Pack 7

## NOPE, IT STILL DOESN'T WORK

Delete everything from your Jackbox Party Pack 7/games folder, then find the "Verify integrity of game files..." button and click it.

## The .EXE file is way too slow

If the .EXE is too slow for you, you can just follow the steps for cloning the repository and using the .PY file.

## I don't have Windows, and I don't want to install python

As of right now, you're just going to have to install python and install jppc.py on your OS of choice. I haven't figured out how to make distributables for other OSes yet.

### Important note if you've clicked "verify integrity of game files..."

That means all your custom prompts have been removed from the game. To get your custom prompts back, you'll have to follow a couple of steps.
1. Make a backup of custom_content.json
2. Use the "Reset All Custom Content" option available in the menu.
3. Use the import feature, and select the backup you made of custom_content.json.

## Why does this program sometimes use weird names for each game?

I have here a handy conversion for the games and their weird names:
- BlankyBlank - Blather 'Round
- Everyday - Devils and the Details
- JackboxTalks - Talking Points 
- World Champions - Champ'd Up
- Quiplash3 - Quiplash 3

The program does this because that's what the folders for each game are called.

# Editing content

If you want to change specific parts of your content or delete content, you're going to want to edit that content in the View/Edit Content option. You should note that you can select multiple pieces of content to edit, view, or delete.

# Importing content

The import content feature cannot import certain custom files like .JPGs or .OGGs. It'll import things like prompts just fine, but to import things like images and sounds you'll need to go into the editing menu and edit each piece of content that contains a custom file (like a .JPG or .OGG) and add in the image manually.

# Using only custom content in a game

This is not at all recommended. If you have less than a certain amount of content for the game to pull from, it might just break entirely. It's better to mix in your custom content with the existing content. If you still want to only use custom content for your game, you can use the "Only Use Custom Content" option from the main menu to delete all existing game content.

# Making custom responses to specific text for Quiplash 3:

So, you may notice that in Quiplash 3 the announcer will sometimes react to a specific prompt. You can do this too!

NOTE: This only works for Round 1 and 2 questions. You can't have custom responses for Final Round questions. 

Okay, I'm note entirely sure how this works, but say you have a prompt like:

`Oh no, my dog ate my <BLANK>!`

And you want a specific response if someone says "homework".

In the "What to filter field", you'd put:

`<PRONOUN> Homework|<PROUNOUN> homework|hw|<PRONOUN> hw`

You should already know that Jackbox uses tags like <ANYPLAYER> and <BLANK> for questions, and so for their responses to specific answers,
they use the tags <ARTICLE> (Like "the", "a", "an", "a massive", "a lot of", etc.), <PRONOUN> (Like "I", "My", "His", "Her", "I've got", "this", "that", etc.), and I believe <VERB> (Like "having a", "craving a", "needing a", "downing a", "guzzling") (I haven't seen a custom response that uses <VERB> yet, but the game can detect the kinds of examples I just gave, I think. You'll have to test it out for yourself.). Jackbox will also separate possible answers by a "|" sign.

You should also try to anticipate alternate answers, like abbreviations or misspellings. Let's look at another example.

For the question "What skin tags probably taste like", Jackbox has:

`<ARTICLE>  chicken | chicken|<ARTICLE>  chiken | chiken|<ARTICLE> chikin | chikin`

as a filter, trying to detect things like "A chicken", "chicken", "A chiken", "chiken", etc.

Note the spaces in between the "|" signs in the example above. It's just a formatting thing, I don't think it really matters. You can add spaces only if you want to.

Once you've added the filtering, then you can add your response audio as a .ogg file. Jackbox also requires a transcript of your response for captioning purposes (I think),
so you should write out what you've said in the "Transcript of your response: " field.
