# What is this?

It's a program for adding your own content (stuff like prompts, certain sound files, etc.) to the Jackbox Party Pack 7.

WARNING: I've really only tested this on Windows, it might not work on other operating systems. 

You may also want to keep a backup of the Jackbox Party Pack 7's files if you're not using Steam,
otherwise you're going to have to uninstall and reinstall your entire game if something goes wrong.

## Some screenshots of custom content in the Jackbox Party Pack 7:

![Talking Points](https://raw.githubusercontent.com/ambiguousname/jackbox-custom-content/main/screenshots/TalkingPoints2.PNG)
![Quiplash3](https://raw.githubusercontent.com/ambiguousname/jackbox-custom-content/main/screenshots/Quiplash3.PNG)

Check out the video [demo](https://youtu.be/4YO2SM21eIo).

# Install instructions

## If you're running on Windows:
- Go to the [releases](https://github.com/ambiguousname/jackbox-custom-content/releases) page and download the .ZIP file.
- Extract the contents of the .ZIP file into its own folder.
- Run the executable. You will be walked through setup.
- Please read the rest of this Readme for clearer instructions if you ever get confused.

## Otherwise:
- Clone this repository
- You can build it using Cargo: https://doc.rust-lang.org/cargo/getting-started/index.html
- Then run: `cargo run` once cargo is installed.
- Please read the rest of this Readme for clearer instructions if you ever get confused.

I will try to work on adding additional OSes when I can!

## To add the sample custom content:
Before installing, the sample content does have some content that could be considered adult. So be warned if you want to import it.

- Install and run the Jackbox Custom Content executable at least once.
- Download this repository and extract the "Sample Content" folder somewhere.
- You can either select: File->Import Mod and select the Sample Content folder, or:
  - Find the mods folder in the same folder as the Jackbox Custom Content executable
  - Extract the "Sample Content" folder to the mods folder.
- The content should show up in the mods side pane now.

Just so you know, the sample custom content is meant to be played with other content in the mix (there isn't enough sample custom content to last a full game with 8 players).

## Common things to watch out for:
### Tags

The \<BLANK\> tag is used in prompts to signify a fill in the blank question. Like with `My <BLANK> is too big!`.

The \<ANYPLAYER\> tag is used to signify that this prompt uses the name of some random player in the game, like with `Hey, <ANYPLAYER> has a problem of too much <BLANK>.`. To use the \<ANYPLAYER\> tag, you should also check the checkbox that says "Includes Player Name", otherwise the \<ANYPLAYER\> tag won't work.

### Custom Files

You'll often see a "Browse" button for some games, which allows you to add your own custom files (like .OGG or .JPG files) for the game. These are entirely optional, but if you're going to make a custom file, please save your custom files into a folder called "external content" located in the same file as jppc.py or Jackbox Content Custom.exe. This way, if you're going to share your content with someone else, the software will recognize that you want this content to be shared with other people. Just make sure to share both "custom_content.json" and the "external content" folder (and make sure they're in the same location).

If you're confused about anything else, please read this Readme. Please.

# Features

## Editing content

If you want to change specific parts of your content or delete content, you're going to want to edit that content in the View/Edit Content option. You should note that you can select multiple pieces of content to edit, view, or delete.

### The "Make New Content" Button

If you've made changes to content, but you want to save those changes to a new piece of content (rather than editing the existing piece of content), push this button to make your changes into a new piece of content.

## Importing content and/or Sharing Content

If you copy the external_content.json file that appears in the same folder as jppc.py or Jackbox Party Pack Custom.exe and send those files to someone else, they can then import that content, using the "Import/Reimport Content" option.

The import content feature will only import certain custom files like .JPGs or .OGGs if those files are stored in the folder ./external content/, if that folder is in the same location as jppc.py or Jackbox Party Pack Custom.exe. 

Additionally, you may have to edit imported content for Champ'd Up, since certain prompts are connected to other prompts with specific IDs, which may be changed on import. 

## Using only custom content in a game

This is not at all recommended. If you have less than a certain amount of content for the game to pull from, the game will not continue. It's better to mix in your custom content with the existing content. If you still want to only use custom content for your game, you can use the "Only Use Custom Content" option from the main menu to delete all existing game content.

This option isn't recommended for any game unless that game has at least the number of content described here:
- Blather 'Round - At least 36 Words. 12 Words of "easy" difficulty, 12 of "medium" difficulty, and 12 of "hard" difficulty.
- Champ'd Up - At least 8 Round 1 Prompts, 8 Round 2 Prompts, 8 Round 2.5 Prompts
- Quiplash 3 - At least 8 Round 1 Prompts, 8 Round 2 Prompts, 1 Round 3 Prompt
- Talking Points - At least 24 Prompts, 24 Pictures, 24 Slide Transitions

# Potential Questions/Problems

## Doesn't Quiplash 3 already have a way for you to make your own questions?

Yes, but there are a few differences. For one: making your own "episodes" in Quiplash 3 means you have to manually select them. If you add custom questions using this program, they'll get inserted into the normal rotation of questions. For two: using this program will allow you to add more stuff not available in Quiplash 3's "episodes" feature, like audio files to read the questions and custom responses to certain answers. And this program can be used for more games than Quiplash 3.

## Help, I accidentally deleted a mod!
On Windows at least, you can recover from the Recycle Bin. Not sure about other operating systems.

## I accidentally added content multiple times, can I remove it?

You can delete any content you make with Edit->Delete Content.

## HELP, EVERYTHING IS BROKEN AND/OR I REMOVED ALL NON-CUSTOM CONTENT AND CAN'T GET IT BACK

If you're using Steam, go to the Jackbox Party Pack 7 in your Steam Library. Right click on the game's icon or name, click "Properties".
In the popup window, click on "Local Files". Then click "Verify integrity of game files..." That should fix everything.

### Important note if you've clicked "verify integrity of game files..."

That means all your custom prompts have been removed from the game. To get your custom prompts back, you'll have to follow a couple of steps.
1. Use the "Import/Reimport" option available in the menu
2. Select "custom_content.json" from the file browser.
3. Import.

## EVERYTHING'S STILL BROKEN

Uninstall and reinstall the Jackbox Party Pack 7

## NOPE, IT STILL DOESN'T WORK

Delete everything from your Jackbox Party Pack 7/games folder, then find the "Verify integrity of game files..." button and click it.

## The .EXE file is way too slow

If the .EXE is too slow for you, you can just follow the steps for cloning the repository and using the .PY file.

## I don't have Windows, and I don't want to install Python

As of right now, you're just going to have to install Python and install jppc.py on your OS of choice. I'm using PyInstaller to compile my code, and PyInstaller is not able to cross-compile.

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

# Custom content guides

## Making custom content for Blather Round

Blather 'Round is one of the most customizable games for the Jackbox Party Pack 7. As such, it has a lot of confusing content options. Here's a description of what each content means, along with some descriptions for the options:

### Word
The word that the player is trying to guess.
#### Word/Phrase Category 
The category to describe the word/phrase. Default options are person, place, thing, or story. If you want to add your own broad category, see Category.
#### Subcategory
You can put anything you want here. Just add one word that adds a little bit more detail than the previous category (e.g., `tv` for `Yu Gi Oh!`, `athlete` for `LeBron James`, `animal` for `Walrus`). You should use an existing subcategory (See [the wiki](https://github.com/ambiguousname/jackbox-custom-content/wiki/Possible-Blather-Round-Subcategories-(Sorted-by-category))). If you're going to make up your own subcategory, please see Descriptor for making your own descriptive sentences.
#### Difficulty
I'm pretty sure you can put whatever you want, but it's recommended to put `easy` for things that are fairly common knowledge (e.g., Australia, Office Space), `medium` for things that require more specific knowledge (e.g., Walrus, Marianas Trench, Les Mis), and `hard` for things that require very specific knowledge (e.g., Diff'rent Strokes, Mr. Snuffleupagus)
#### Forbidden Words
Hardly ever used, but if you have some common words that occur in your word/phrase or some really good descriptors (Like `murder` in `Murder She Wrote` or `big` and `dude` in `Big Lebowsky`), then you should put in those words here.
#### Tailored Words
Words that are tailor made to more accurately describe the word/phrase. First describe the descriptor (put into brackets: \<descriptor\>), then the specific word (separate by |, so: `<descriptor>|word`). What are the categories/words? Well, you can make your own in the Descriptor menu. If you want to use pre-existing words, search [the wiki](https://github.com/ambiguousname/jackbox-custom-content/wiki/Blather-Round-Desciptor-Words-List). You should see each descriptor (listed under `name`), along with a list of words to match that descriptor (for instance, if I had `Pompeii`, I would write `<emotion-bad>|sad|<building>|structure|<land>|land|<texture-complex>|firey|<abstract-concept>|tourism|<building-complex>|ruin`, etc.) 

### Category 
A *broad* category meant to describe the general idea of a word (ideally person/place/thing/story work well, so making a new category isn't recommended)
#### Structures 
The sentence structures used to give hints about what the thing is about. Use \<descriptor\> tags (e.g., \<emotion-bad\>, \<building\>) for each thing you have to fill in the blank for (again, go to [the wiki](https://github.com/ambiguousname/jackbox-custom-content/wiki/Blather-Round-Desciptor-Words-List) to see the words you can use, or add your own with Descriptor). Separate each entry by |.

### Descriptor 
You have three options: Describing Adjectives/Nouns/Verbs to apply to a category, sentences to respond to other people's guesses (like `It's very similar to ____!`), or descriptor words meant for \<descriptor\> tags (to be used in the Tailored Words section for a Word). The steps for making each are similar.
#### Descriptor name
How you name the descriptor will (I think) determine how that descriptor is used.
- If I'm making a specific group of words (adjectives, nouns, or verbs) that pair with a category, I'd name the Descriptor `CATEGORY-VERB/ADJECTIVE/NOUN-SIMPLE/COMPLEX`. Where you write in the category name, whether you're using a verb, adjective, or noun, and whether the list of words is simple or complex. Something is considered `complex` if it has relatively simple words (I trust you to use your own judgement here). So if I were making a list of verbs that matched with category `story` with verbs like `runs`, `eats`, `lives with`, etc., I'd call it `story-verb-simple`
- If I'm making a responding sentence to a subcategory, I name it: `response-sentence-CATEGORY-SUBCATEGORY`. You can remove the `-SUBCATEGORY` if you want to make a responding sentence to an overall category. So for instance, if I wanted to list possible responding sentences to something that has a category of `place` and `tv`, I'd write `response-sentence-place-tv`.
- If I'm making a descriptor words for a \<descriptor\> tag (to be used by Category and Word content), I'd call it whatever I'd like (as long as it's hyphenated). So if I were to make a bunch of words describing odors I'd call it `smells-simple`, or something like that.
#### Words List
The list of words (or sentences) that you're using for the Descriptor. If you're writing a list of words, you can use \<descriptor\> tags to refer to other descriptors. Separate each word/sentence with |. If you consider a word or sentence to be essential to a descriptor, add a `T|` in front to signify that the word/sentence  is essential:
- If I'm writing something for `story-verb-simple`, I write something like: `runs|eats|lives with|T|discovers|T|learns`, etc.
- If I'm writing something for `response-sentence-place-tv`, I'd write something like: `T|It's something like|T|It's a fictional version of|T|It reminds me of`, etc.
- If I'm writing something for `smells-simple`, I'd write something like `gross|<taste-complex>|nasty|lemony`
#### Max Choices
If a player is making a selection on what words to choose, is there a set limit to how much they get to pick? (Please write something like 1, 2, or 3)
- For something like `story-verb-simple`, you should set this to 1, 2, or 3 since you're probably going to use a verb once in a sentence (1), an adjective maybe three times (3), and a noun maybe twice (2).
- For something like `response-sentence-place-tv`, set this to 1, since you're only going to pick one sentence.
- For something like `smells-simple`, don't set this at all, since the game will automatically decide a limit for descriptors regarding \<descriptor\> tags.
#### Placeholder text 
Generally, the placeholder text used when you can't get a sentence or a word there. Usually, it's something like `blank` (for non plural words), `blanks` (for plural words), and `blanky` (for sentences).
- For `story-verb-simple`, the placeholder would be `blanks` (since almost every word/phrase is plural)
- For `response-sentence-place-tv`, the placeholder would be `blanky` (since everything in the words list is a sentence)
- For `smells-simple`, the placeholder would be `blank` (since every word is singular)

## Making custom responses to specific text for Quiplash 3:

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