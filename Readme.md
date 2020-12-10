# What is this?

It's a program for adding your own content (stuff like prompts, certain sound files, etc.) to the Jackbox Party Pack 7.

WARNING: I've really only tested this on Windows, it might not work on other operating systems. 

You may also want to keep a backup of the Jackbox Party Pack 7's files if you're not using Steam,
otherwise you're going to have to uninstall and reinstall your entire game if something goes wrong.

## Some screenshots of custom content in the Jackbox Party Pack 7:

![Talking Points](https://raw.githubusercontent.com/ambiguousname/jackbox-custom-content/main/screenshots/TalkingPoints2.PNG)
![Quiplash3](https://raw.githubusercontent.com/ambiguousname/jackbox-custom-content/main/screenshots/Quiplash3.PNG)

# Install instructions

## If you don't have python installed and are running on Windows:
- Go to the releases page and download the .ZIP file: https://github.com/ambiguousname/jackbox-custom-content/releases
- Extract the contents of the .ZIP file into the "games" directory of your Jackbox Party Pack 7 install folder. (For steam on Windows: C:\Program Files (x86)\Steam\steamapps\common\The Jackbox Party Pack 7\games)
- Run "Jackbox Party Pack Custom.exe".

## If you do have python installed:
- Clone this repository
- Move "jppc.py" to the "games" directory of your Jackbox Party Pack 7 install folder. (For steam on Windows: C:\Program Files (x86)\Steam\steamapps\common\The Jackbox Party Pack 7\games)
- Run jppc.py in the terminal of your choice.

## To add the sample custom content:
Before installing, the sample custom content does contain some adult... jokes (I wrote them very late in the evening, the level of comedy will vary significantly)? So be warned if you want to import it.

- Add Jackbox Party Pack Custom.exe or jppc.py to the "games" directory of the Jackbox Party Pack 7 folder.
- Run jppc.py or Jackbox Party Pack Custom.exe
- Select the "Import/Reimport Content" option.
- Select the "sample_custom_content.json" file.
- Click "Import".
- To view your new content, click on "View/Edit Content", then click "All Games".

Just so you know, the sample custom content is meant to be played with other content in the mix (there isn't enough sample custom content to last a full game with 8 players). While you can use the "Only Custom" menu option to get rid of the game's own content files and play with only the sample content, it's not recommended.

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
1. Use the "Import/Reimport" option available in the menu
2. Select "custom_content.json" from the file browser.
3. Import.

## Why does this program sometimes use weird names for each game/content?

I have here a handy conversion for the games and their weird names:
- BlankyBlank - Blather 'Round
  - BlankyBlankPasswords - Word
  - BlankyBlankSentenceStructures - Category
  - BlankyBlankWordLists - Descriptor
- JackboxTalks - Talking Points 
  - JackboxTalksPicture - Picture
  - JackboxTalksTitle - Prompt
  - JackboxTalksSignpost - Slide Transition
- Quiplash3 - Quiplash 3
  - Quiplash3Round1Question - Round 1 Question
  - Quiplash3Round2Question - Round 2 Question
  - Quiplash3FinalQuestion - Final Round Question
  - Quiplash3SafetyQuips - Safety Quips
- World Champions - Champ'd Up
  - WorldChampionsRound - Round 1
  - WorldChampionsRoundSecondHalfA - Round 2
  - WorldChampionsRoundSecondHalfB - Round 2.5

The program does this because that's what the folders for each game are called.

## Why isn't Devils and the Details included on that list?

An excellent question. For now, I'm not going to bother supporting custom Devils and the Details content for a few reasons:
1. The Devils and the Details' game files have a different file structure compared to every other game in the party pack, meaning it would be harder for me to add support
2. Those game files use a language called "EVERYDASIC" (similar to BASIC, ha ha, get it?), which I'm going to have to deconstruct and fiddle around with if I want to add support.
3. I don't think the game is that great anyway, and so all that effort for Devils and the Details doesn't seem worth it.

# Editing content

If you want to change specific parts of your content or delete content, you're going to want to edit that content in the View/Edit Content option. You should note that you can select multiple pieces of content to edit, view, or delete.

## The "Make New Content" Button

If you've made changes to content, but you want to save those changes to a new piece of content (rather than editing the existing piece of content), push this button to make your changes into a new piece of content.

# Importing content

The import content feature will only import certain custom files like .JPGs or .OGGs if those files are stored in the folder ./external content/, if that folder is in the same location as jppc.py or Jackbox Party Pack Custom.exe. 

Additionally, you may have to edit imported content for Champ'd Up, since certain prompts are connected to other prompts with specific IDs, which may be changed on import. 

# Using only custom content in a game

This is not at all recommended. If you have less than a certain amount of content for the game to pull from, the game will not continue. It's better to mix in your custom content with the existing content. If you still want to only use custom content for your game, you can use the "Only Use Custom Content" option from the main menu to delete all existing game content.

# Making custom content for Blather Round

NOTE: For reasons you'll see in a second, it's not recommended that you use the "Only Custom Content" option for Blather 'Round.

Blather 'Round is one of the most customizable games for the Jackbox Party Pack 7. As such, it has a lot of confusing content options. Here's a description of what each content means, along with some descriptions for the options:

## Word
The word that the player is trying to guess.
### Word/Phrase Category 
The category to describe the word/phrase. Default options are person, place, thing, or story. If you want to add your own broad category, see Category.
### Subcategory
You can put anything you want here. Just add one word that adds a little bit more detail than the previous category (e.g., `tv` for `Yu Gi Oh!`, `athlete` for `LeBron James`, `animal` for `Walrus`). You should use an existing subcategory (See [the wiki](https://github.com/ambiguousname/jackbox-custom-content/wiki/Possible-Blather-Round-Subcategories-(Sorted-by-category))). If you're going to make up your own subcategory, please see Descriptor for making your own descriptive sentences.
### Difficulty
I'm pretty sure you can put whatever you want, but it's recommended to put `easy` for things that are fairly common knowledge (e.g., Australia, Office Space), `medium` for things that require more specific knowledge (e.g., Walrus, Marianas Trench, Les Mis), and `hard` for things that require very specific knowledge (e.g., Diff'rent Strokes, Mr. Snuffleupagus)
### Forbidden Words
Hardly ever used, but if you have some common words that occur in your word/phrase or some really good descriptors (Like `murder` in `Murder She Wrote` or `big` and `dude` in `Big Lebowsky`), then you should put in those words here.
### Tailored Words
Words that are tailor made to more accurately describe the word/phrase. First describe the descriptor (put into brackets: \<descriptor\>), then the specific word (separate by |, so: `<descriptor>|word`). What are the categories/words? Well, you can make your own in the Descriptor menu. If you want to use pre-existing words, search [the wiki](https://github.com/ambiguousname/jackbox-custom-content/wiki/Blather-Round-Desciptor-Words-List). You should see each descriptor (listed under `name`), along with a list of words to match that descriptor (for instance, if I had `Pompeii`, I would write `<emotion-bad>|sad|<building>|structure|<land>|land|<texture-complex>|firey|<abstract-concept>|tourism|<building-complex>|ruin`, etc.) 

## Category 
A *broad* category meant to describe the general idea of a word (ideally person/place/thing/story work well, so making a new category isn't recommended)
### Structures 
The sentence structures used to give hints about what the thing is about. Use \<descriptor\> tags (e.g., \<emotion-bad\>, \<building\>) for each thing you have to fill in the blank for (again, go to [the wiki](https://github.com/ambiguousname/jackbox-custom-content/wiki/Blather-Round-Desciptor-Words-List) to see the words you can use, or add your own with Descriptor). Separate each entry by |.

### Descriptor 
You have three options: Describing Adjectives/Nouns/Verbs to apply to a category, sentences to respond to other people's guesses (like `It's very similar to ____!`), or descriptor words meant for \<descriptor\> tags (to be used in the Tailored Words section for a Word). The steps for making each are similar.
## Descriptor name
How you name the descriptor will (I think) determine how that descriptor is used.
- If I'm making a specific group of words (adjectives, nouns, or verbs) that pair with a category, I'd name the Descriptor `CATEGORY-VERB/ADJECTIVE/NOUN-SIMPLE/COMPLEX`. Where you write in the category name, whether you're using a verb, adjective, or noun, and whether the list of words is simple or complex. Something is considered `complex` if it has relatively simple words (I trust you to use your own judgement here). So if I were making a list of verbs that matched with category `story` with verbs like `runs`, `eats`, `lives with`, etc., I'd call it `story-verb-simple`
- If I'm making a responding sentence to a subcategory, I name it: `response-sentence-CATEGORY-SUBCATEGORY`. You can remove the `-SUBCATEGORY` if you want to make a responding sentence to an overall category. So for instance, if I wanted to list possible responding sentences to something that has a category of `place` and `tv`, I'd write `response-sentence-place-tv`.
- If I'm making a descriptor words for a \<descriptor\> tag (to be used by Category and Word content), I'd call it whatever I'd like (as long as it's hyphenated). So if I were to make a bunch of words describing odors I'd call it `smells-simple`, or something like that.
## Words List
The list of words (or sentences) that you're using for the Descriptor. If you're writing a list of words, you can use \<descriptor\> tags to refer to other descriptors. Separate each word/sentence with |. If you consider a word or sentence to be essential to a descriptor, add a `T|` in front to signify that the word/sentence  is essential:
- If I'm writing something for `story-verb-simple`, I write something like: `runs|eats|lives with|T|discovers|T|learns`, etc.
- If I'm writing something for `response-sentence-place-tv`, I'd write something like: `T|It's something like|T|It's a fictional version of|T|It reminds me of`, etc.
- If I'm writing something for `smells-simple`, I'd write something like `gross|<taste-complex>|nasty|lemony`
## Max Choices
If a player is making a selection on what words to choose, is there a set limit to how much they get to pick? (Please write something like 1, 2, or 3)
- For something like `story-verb-simple`, you should set this to 1, 2, or 3 since you're probably going to use a verb once in a sentence (1), an adjective maybe three times (3), and a noun maybe twice (2).
- For something like `response-sentence-place-tv`, set this to 1, since you're only going to pick one sentence.
- For something like `smells-simple`, don't set this at all, since the game will automatically decide a limit for descriptors regarding \<descriptor\> tags.
## Placeholder text 
Generally, the placeholder text used when you can't get a sentence or a word there. Usually, it's something like `blank` (for non plural words), `blanks` (for plural words), and `blanky` (for sentences).
- For `story-verb-simple`, the placeholder would be `blanks` (since almost every word/phrase is plural)
- For `response-sentence-place-tv`, the placeholder would be `blanky` (since everything in the words list is a sentence)
- For `smells-simple`, the placeholder would be `blank` (since every word is singular)

# Making custom responses to specific text for Quiplash 3:

So, you may notice that in Quiplash 3 the announcer will sometimes react to a specific prompt. You can do this too!

This only works for Round 1 and 2 questions. You can't have custom responses for Final Round questions. 

Let's say you have a prompt like:

`Oh no, my dog ate my <BLANK>!`

And you want a specific response if someone says "homework".

In the "What to filter field", you'd put:

`<PRONOUN> Homework|<PROUNOUN> homework|hw|<PRONOUN> hw`

You should already know that Jackbox uses tags like \<ANYPLAYER\> and \<BLANK\> for questions, and so for their responses to specific answers,
they use the tags \<ARTICLE\> (Like "the", "a", "an", "a massive", "a lot of", etc.), \<PRONOUN\> (Like "I", "My", "His", "Her", "I've got", "this", "that", etc.), and <VERB\> (Like "having a", "craving a", "needing a", "downing a", "guzzling"). To see what kinds of answers tags like \<ARTICLE\> will give, check [the wiki](https://github.com/ambiguousname/jackbox-custom-content/wiki). You should also separate possible answers by a "|" sign.

You should also try to anticipate alternate answers, like abbreviations or misspellings. Let's look at another example.

For the question "What skin tags probably taste like", Jackbox has: `<ARTICLE>  chicken | chicken|<ARTICLE>  chiken | chiken|<ARTICLE> chikin | chikin` as a filter, trying to detect things like "A chicken", "chicken", "A chiken", "chiken", etc.

Note the spaces in between the "|" signs in the example above. It's just a formatting thing, I don't think it really matters. You can add spaces only if you want to.

Once you've added the filtering, then you can add your response audio as a .ogg file. Jackbox also requires a transcript of your response (I think for captioning purposes), so you should write out what you've said in the "Transcript of your response: " field.