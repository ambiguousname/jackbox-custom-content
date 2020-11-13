import PySimpleGUI as sg
import json
import os
from shutil import copyfile, rmtree

#TODO:
# Test safety quips
# Test importing content
# Test Talking Points Content
# Add example images in the Readme
# Add feature to delete everything but custom content

def id_gen(custom_values): #custom_values should be a dict that passes on any other identifying information for the user
    ids = None #Start IDs from 100k (to make it distingusihable from other IDs), go from there.
    id_dict = None
    if os.path.exists("./custom_content.json"):
        ids = open("./custom_content.json", 'r+')
        id_dict = json.load(ids)
    else:
        ids = open("./custom_content.json", 'w')
        id_dict = {}
    
    _id = None
    if type(id_dict) == None:
        _id = "100000"
    else:
        _id = str(100000 + len(id_dict.keys()))
    custom_values.update({"id": _id}) #Need to store the id twice so that things can work. The .jet files need a reference to the id.
    id_dict[_id] = {"id": _id, "values": custom_values}
    ids.seek(0)
    ids.truncate()
    new_json = json.dumps(id_dict)
    ids.write(new_json)
    ids.close()
    return _id

class CustomContent(object):
    def __init__(self, values, game, content_type, descriptor_text, _id=None): #values, game, content_type, descriptor_text, _id=None
        self.values = {"game": game, "content_type": content_type, "descriptor_text": descriptor_text}
        self.values.update(values)
        if _id == None: #Are we using an existing id?
            self.id = id_gen(self.values)
        else:
            self.id = _id
        self.values.update({"id": self.id})
    
    def write_to_json(self, p=None, delete=False): #Right now, add_custom_files doesn't support custom file paths. Only write_to_json supports custom paths for data.jet files.
        path = "" 
        if p:
            path = p
        else:
            path = "./" + self.values["game"] + "/content/" + self.values["content_type"] + ".jet"
        if os.path.exists(path): #Are we making a new .JSON file, or are we appending to an existing .JSON file?
            jf = open(path, "r", encoding="utf-8")
            json_file = json.load(jf)
            if delete == True:
                json_file["content"].remove(self.values)
            else:
                if self.values in json_file["content"]:
                    json_file["content"].remove(self.values)
                json_file["content"].append(self.values)
            jf.close()
            #Close and reopen to write, because writing with utf-8 encoding gets... weird.
            jf = open(path, "w")
            jf.write(json.dumps(json_file))
            jf.close()
        else:
            jf = open(path, "w")
            if delete != True:
                jf.write(json.dumps(self.values))
            jf.close()
    
    def save_to_custom_content(self): #Save to custom_content.json file which keeps track of everything
        ids = open("./custom_content.json", "r+")
        content = json.load(ids)
        content[self.id].update({"id": self.id, "values": self.values})
        ids.seek(0)
        ids.truncate()
        ids.write(json.dumps(content))
        ids.close()

    def add_custom_files(self, *args, **kwargs): #Construct the path from what we already know.
        path = ""
        if "path" in kwargs:
            path = kwargs["path"]
        else:
            path = "./" + self.values["game"] + "/content/" + self.values["content_type"] + "/" + self.id + "/"
        if os.path.exists(path) and not "path" in kwargs: #If there's a folder here, but we're not selecting a custom path
            rmtree(path)
        if not ("delete" in kwargs and kwargs["delete"] == True):
            if not (os.path.exists(path)):
                os.mkdir(path)
            for file in args:
                if type(file) == dict and "path" in file: #If we're just copying a file
                    if(os.path.exists(file['path'])): #Only add this if the file's path exists.
                        copyfile(file['path'], path + file['name']) #From shutil
                else: #If we're going to be writing a custom file from like a .JSON or whatever.
                    if isinstance(file, CustomContent):
                        if os.path.exists(path + "data.jet"):
                            os.remove(path + "data.jet")
                        file.write_to_json(path + "data.jet")
                    elif 'str' in file: #Just making sure there are no files that have an empty path. "str" is if a file has specific data that we're writing.
                        f = open(path + file['name'], "w+")
                        f.write(file['str'])
                        f.close()

class CustomData(CustomContent):
    def __init__(self):
        super()
        self.values = {"fields": []}
    
    def add_data(self, t, v, n): #What are t, v, n? Depends on the game. t is some random letter thing that I can't for the life of me decipher.
        #v is like text? Like, what someone is saying or what they're going to say, or what's being shown on screen (I think it's for captioning/showing text)
        #And n is usually a descriptor saying what the data point is for.
        self.values["fields"].append({
            "t": t,
            "v": v,
            "n": n
        })

class SelectionWindow():
    def __init__(self, title, layout_list, selector, previous_window = None): #back_closes should be if we replace the "Go Back" button with "Close"
        self.layout_list = layout_list
        self.title = title
        self.layout_list = layout_list
        self.list_key = layout_list[2]
        self.selector = selector
        self.previous_window = previous_window

    def run(self, inputs=None): #Have to add inputs as an argument because the "Ok" event needs to pass a set of values for determining stuff. So the run function needs a second argument, but will never actually use it.
        n_layout = [[sg.Text(self.layout_list[0])], [sg.Listbox(self.layout_list[1], size=(30, 10), select_mode="LISTBOX_SELECT_MODE_SINGLE", key=self.layout_list[2])], [sg.Button('Ok'), sg.Button('Exit' if not self.previous_window != None else 'Go Back')]]
        window = sg.Window(self.title, n_layout)
        while True:
            event, values = window.read()
            if event == sg.WIN_CLOSED or event == "Exit":
                break
            if event == "Ok":
                window.close()
                func = self.selector.get(values[self.list_key][0])
                func(values[self.list_key][0]) #What we need the "inputs" argument for. 
                break
            if event == "Go Back" and self.previous_window:
                window.close()
                window_mapping[self.previous_window].run()
                break
        window.close()
                
#Stuff for file management

def edit_content(selected=None): #Selected goes unused because of how SelectWindow works.
    if os.path.exists("./custom_content.json"):
        ids = open("./custom_content.json", 'r+')
        content = json.load(ids)
        content_list = []
        for item in content:
            content_list.append(content[item]["id"] + ": " + content[item]["values"]["content_type"] + " - " + content[item]["values"]["descriptor_text"])
        layout = [[sg.Text("Choose Content to Edit/Delete:")], [sg.Listbox(content_list, key="content_selection", size=(100, 25), select_mode="LISTBOX_SELECT_MODE_SINGLE")], [sg.Button("Edit"), sg.Button("Delete"), sg.Button("Show Folder"), sg.Button("Go Back")]]
        window = sg.Window("Choose Content to Edit/Delete", layout)
        while True:
            event, values = window.read()
            if event == sg.WIN_CLOSED:
                break
            if event == "Show Folder":
                _id = values["content_selection"][0].split(":")[0]
                existing_data = content[_id]["values"]
                path = os.path.realpath("./" + existing_data["game"] + "/content/" + existing_data["content_type"] + "/" + existing_data["id"])
                if "path" in existing_data:
                    path = existing_data["path"]
                if (os.path.exists(path)):
                    os.startfile(path)
                else:
                    sg.Window("There is no folder containing the content (If the content contains only text (like a safety quip), it's probably just stored ... not in a folder).")
            if event == "Edit":
                _id = values["content_selection"][0].split(":")[0]
                existing_data = content[_id]["values"]
                content_type_mapping[existing_data["content_type"]](selected, existing_data)
            if event == "Delete":
                _id = values["content_selection"][0].split(":")[0]
                custom_content = CustomContent(content[_id]["values"], content[_id]["values"]["game"], content[_id]["values"]["content_type"], content[_id]["values"]["descriptor_text"], _id) #Setting None because values already has the game, type, and descriptor_text.
                #Remove the content from the custom_content JSON file
                content.pop(_id)
                #Remove the content from the game's master .JET file
                custom_content.write_to_json(None, True) #Delete the JSON file, using the pre-existing path.
                #Remove the content's custom folder (will do nothing if one doesn't exist)
                custom_content.add_custom_files(delete=True)
                ids.seek(0)
                ids.truncate()
                if len(content.keys()) != 0:
                    ids.write(json.dumps(content))
                    ids.close()
                else:
                    ids.close()
                    os.remove("./custom_content.json")
                window.close()
                sg.Popup("Content deleted!")
                edit_content() #To update the list of content
            if event == "Go Back":
                window.close()
                main_window.run()
                break
        ids.close()
        window.close()
    else:
        sg.Popup("Sorry, no content to edit.")
        main_window.run()

def import_content(selected=None):
    layout = [[sg.Text("To share content for import, share custom_content.json (from the same folder as Jackbox Party Pack Custom.exe). NOTE: See the readme for importing files like .OGGs or .JPGs.")],
    [sg.Text("If that file has been shared with you, select it here: "), sg.InputText(key="custom-files"), sg.FileBrowse(file_types=((".JSON", "*.json"), ("ALL Types", "*.*")))], [sg.Button("Import"), sg.Button("Go Back")]]
    window = sg.Window("Select File to Import", layout)
    while True:
        event, values = window.read()
        if event == sg.WIN_CLOSED:
            break
        if event == "Import":
            if os.path.exists(values["custom-files"]) and os.path.splitext(values["custom-files"])[1].lower() == ".json":
                if os.path.exists("./custom_content.json"):
                    new_ids = open(values["custom-files"], "r")
                    new_content = json.load(new_ids.read())
                    new_ids.close()
                    ids = open("./custom_content.json", "r+")
                    content = json.load(ids.read())
                    latest_id = int(content.keys().sort()[-1])
                    for i in range(new_content.keys()):
                        n_c = new_content[new_content.keys(i)]
                        content[str(latest_id + i + 1)] = n_c
                        content_type_mapping[n_c["content_type"]](selected, n_c["values"]) #Requires you to manually add in each piece of content.
                    ids.seek(0)
                    ids.truncate()
                    ids.write(json.dumps(content))
                    ids.close()
                else:
                    copyfile(values["custom-files"], "./custom_content.json")
                sg.Popup("Custom content imported. View the files in the edit menu.")
            else:
                sg.Popup("That file doesn't exist, or it isn't a .json file.")
        if event == "Go Back":
            window.close()
            main_window.run()
            break
    window.close()

#Stuff for Quiplash 3

def create_quiplash_data_jet(prompt_content):
    data = CustomData()
    data.add_data("B", "true" if prompt_content.response_filter != "" else "false", "HasJokeAudio") 
    data.add_data("S", prompt_content.response_filter, "Keywords")
    data.add_data("A", "response", "KeywordResponseAudio") #Included even though there might not be response audio
    data.add_data("S", prompt_content.response_narration, "KeywordResponseText")
    data.add_data("B", "true" if prompt_content.values["prompt"] != "" else "false", "HasPromptAudio")
    data.add_data("A", "prompt", "PromptAudio") #I think this is asking for the file name of the audio. I think I can leave this in if the audio doesn't exist, because some prompts don't have response audio, but we include the above line. 
    data.add_data("S", prompt_content.values["prompt"], "Prompt Text")
    data.add_data("S", prompt_content.values["safetyQuips"], "SafetyQuips")
    return data

def round_prompt(selection, existing_data=None):
    layout = [[sg.Text("Prompt Text: "), sg.InputText("Hey, <ANYPLAYER> needs to <BLANK>." if existing_data == None else existing_data["prompt"], size=(50,1), key="text")], [sg.Text("Safety Quip(s) (separate by |): "), sg.InputText("learn how the prompt system works|learn how safety quips work|eat all my garbage" if existing_data == None else existing_data["safetyQuips"], size=(50,1), key="safety-quips")],
    [sg.Checkbox("Includes Player Name", default=(True if existing_data == None else existing_data["includesPlayerName"]), key="player-name"), sg.Checkbox("Contains Adult Content", default=(False if existing_data == None else existing_data["x"]), key="x"), sg.Checkbox("Content is US specific", default=(False if existing_data == None else existing_data["us"]), key="us")],
    [sg.Text(".ogg files of you reading things (Optional):")], [sg.Text(".ogg of you saying the prompt: "), sg.InputText(key="prompt"), sg.FileBrowse(file_types=((".OGG", "*.ogg"), ("ALL Files", "*.*")))],
    [sg.Text("Add a response to specific text (Very optional, see Readme for information):")],
    [sg.InputText(key="response"), sg.FileBrowse(file_types=((".OGG", "*.ogg"), ("ALL Files", "*.*")))], 
    [sg.Text("What to filter (See Readme): "), sg.InputText(key="response-filter")],
    [sg.Text("Transcript of your response: "), sg.InputText(key="response-narration")],
    [sg.Button("Make a prompt"), sg.Button("Go Back")]]
    window = sg.Window(("Round " + selection[-1] + " Prompt") if existing_data == None else existing_data["id"], layout)
    while True:
        event, values = window.read()
        if event == sg.WIN_CLOSED:
            break
        if event == "Go Back":
            window.close()
            if existing_data == None:
                quiplash_prompt.run()
            break
        if event == "Make a prompt":
            prompt = CustomContent({
                "includesPlayerName": values["player-name"],
                "prompt": values["text"],
                "safetyQuips": values["safety-quips"].split("|"), #We make an array because that's how they're formatted in the master .jet file.
                "x": values["x"],
                "us": values["us"]
            }, "Quiplash3", ("Quiplash3Round" + selection[-1] + "Question") if existing_data == None else existing_data["content_type"], values["text"], None if existing_data == None else existing_data["id"])
            prompt.response_filter = values["response-filter"]
            prompt.response_narration = values["response-narration"]
            prompt.write_to_json() #Get round number from the choice passed through in quiplash_prompt
            #Write the data.jet file to be added:
            data = create_quiplash_data_jet(prompt)
            prompt.add_custom_files({"path": values["prompt"], "name": "prompt.ogg"}, {"path": values["response"], "name": "response.ogg"}, data)
            sg.Popup("Prompt created, ID: " + prompt.id)
    window.close()

def round_prompt_final(selection, existing_data=None): #I'm making this separate because it's just easier to do this than to explain the last round prompt syntax.
    layout = [[sg.Text("Prompt Text: "), sg.InputText("Three things a stranger would say about <ANYPLAYER>." if existing_data == None else existing_data["prompt"], key="lastround-prompt")], [sg.Text("Safety Quip(s) (separate by |): "), sg.InputText("not|funny|didn't laugh|get|out of|my face|learn|how the safety quips|work" if existing_data == None else existing_data["safetyQuips"], key="lastround-safety-quips")],
    [sg.Checkbox("Includes Player Name", default=(True if existing_data == None else existing_data["includesPlayerName"]), key="player-name"), sg.Checkbox("Contains Adult Content", default=(False if existing_data == None else existing_data["x"]), key="x"), sg.Checkbox("Content is US specific", default=(False if existing_data == None else existing_data["us"]), key="us")],
    [sg.Text(".ogg file of you reading the prompt (Optional):")], [sg.InputText(key="prompt"), sg.FileBrowse(file_types=((".OGG", "*.ogg"), ("ALL Files", "*.*")))],
    [sg.Button("Make a prompt"), sg.Button("Go Back")]]
    window = sg.Window("Make a Quiplash 3 Final Round Prompt" if existing_data == None else existing_data["id"], layout)
    while True:
        event, values = window.read()
        if event == sg.WIN_CLOSED:
            break
        if event == "Go Back":
            window.close()
            if existing_data == None:
                quiplash_prompt.run()
            break
        if event == "Make a prompt":
            #Safety quips for the final round are a bit weird:
            safety_quips = values["lastround-safety-quips"].split("|")
            formatted_quips = []
            for i in range(len(safety_quip)):
                if not (i + 3 > len(safety_quip)):
                    formatted_quips.append(safety_quips[0] + "|" + safety_quips[1] + "|" + safety_quips[2])
            prompt = CustomContent({
                "includesPlayerName": values["player-name"],
                "prompt": values["lastround-prompt"],
                "safetyQuips": formatted_quips,
                "x": values["x"],
                "us": values["us"]
            }, "Quiplash3", "Quiplash3FinalQuestion", values["lastround-prompt"], None if existing_data == None else existing_data["id"])
            prompt.write_to_json()
            prompt.response_filter = ""
            prompt.response_narration = ""
            data = create_quiplash_data_jet(prompt)
            prompt.add_custom_files({"path": values["prompt"], "name": "prompt.ogg"}, data)
            sg.Popup("Prompt created, ID: " + prompt.id)
    window.close()

def safety_quip(selection, existing_data=None):
    layout = [[sg.Text("Safety Quip Text (Should be generic): "), sg.InputText("" if existing_data == None else existing_data["value"], key="safety-quip")], [sg.Button("Make Quip"), sg.Button("Go Back")]]
    window = sg.Window("Make a Safety Quip" if existing_data == None else existing_data["id"], layout)
    while True:
        event, values = window.read()
        if event == sg.WIN_CLOSED:
            break
        if event == "Make Quip":
            quip = CustomContent({"value": values["safety-quip"]}, "Quiplash3", "Quiplash3SafetyQuips", values["safety-quip"], None if existing_data == None else existing_data["id"])
            quip.write_to_json()
            sg.Popup("Safety Quip Created. ID: " + quip.id)
        if event == "Go Back":
            window.close()
            if existing_data == None:
                quiplash_3.run()
            break
    window.close()

quiplash_prompt = SelectionWindow("Choose a Round", ["Choose a round.", ("Round 1", "Round 2", "Final Round"), "quiplash3_round_number"], {
    "Round 1": round_prompt,
    "Round 2": round_prompt,
    "Final Round": round_prompt_final
}, "quiplash_3")

quiplash_3 = SelectionWindow("Quiplash 3 Content Selection", ["Please select the type of content", ("Prompt", "Safety Quip"), "quiplash3_content_type"], {
    "Prompt": quiplash_prompt.run,
    "Safety Quip": safety_quip
}, "create_content")

#Stuff for Talking Points

def talking_points_picture(selection=None, existing_data=None):
    layout = [[sg.Text("Choose a .JPG file to add: "), sg.InputText("" if existing_data == None else os.getcwd() + "/JackboxTalks/content/JackboxTalksPicture/" + existing_data["id"] + ".jpg", key="photo"), sg.FileBrowse(file_types=((".JPG", "*.jpg"), ("ALL Files", "*.*")))],
    [sg.Text("Low Res .JPG (recommended, will use higher-res picture if not given): "), sg.InputText("" if existing_data == None else os.getcwd() + "/JackboxTalks/content/JackboxTalksPictureLow/" + existing_data["id"], key="low_res_photo"), sg.FileBrowse(file_types=((".JPG", "*.jpg"), ("ALL Files", "*.*")))], 
    [sg.Text("Description of Picture: "), sg.InputText("" if existing_data == None else existing_data["name"], key="photo_description")], [sg.Checkbox("Picture contains adult content", default=False if existing_data == None else existing_data["x"], key="x")], [sg.Button("Add"), sg.Button("Go Back")]]
    window = sg.Window("Add a Picture" if existing_data == None else existing_data["id"], layout)
    while True:
        event, values = window.read()
        if event == sg.WIN_CLOSED:
            break
        if event == "Go Back":
            window.close()
            if existing_data == None:
                talking_points.run()
            break
        if event == "Add":
            if os.path.exists(values["photo"]) and os.path.splitext(values["photo"])[1].lower() == ".jpg":
                picture = values["photo"]
                low_res = values["low_res_photo"]
                if not (os.path.exists(low_res) and os.path.splitext(low_res)[1].lower() == ".jpg"):
                    low_res = picture
                #Only using one content to write this, otherwise editing this file is going to get messy.
                picture_content = CustomContent({
                    "altText": values["photo_description"],
                    "name": values["photo_description"],
                    "x": values["x"]
                }, "JackboxTalks", "JackboxTalksPicture", values["photo_description"])
                picture_content.values["path"] = os.getcwd() + "\JackboxTalks\content\JackboxTalksPicture"
                picture_content.write_to_json()
                #Write high-res picture
                picture_content.add_custom_files({"path": picture, "name": picture_content.id + ".jpg"}, path="./JackboxTalks/content/JackboxTalksPicture/")
                #Write low_res picture in a different folder
                picture_content.add_custom_files({"path": low_res, "name": picture_content.id + ".jpg"}, path="./JackboxTalks/content/JackboxTalksPictureLow/")
                #Save to custom_content.json
                picture_content.save_to_custom_content()
                sg.Popup("Picture Added, ID: " + picture_content.id)
            else:
                sg.Popup("You didn't select a valid file.")
    window.close()

def talking_points_prompt(selection=None, existing_data=None):
    slide_transitions = "m,For those of you questioning my reasons, I was motivated by this...|m,For those of you who object, here's why you're all powerless to stop me...|m,If you're concerned about permissions, I have all the power I need from this...|e,Now for the Finale: What you're about to see next will ultimately prove my superiority...|m,What I'm about to say is actually banned in about 20 countries, so pay close attention...|e,For those of you at home, imitate exactly what you're about to hear and see...|e,Now it's flex time, and I'm going to flex with this...|e,I have no words for what you're about to witness, only vague and confusing noises/hand movements...|m,For this amazing feat, I will make use of this as a centerpiece...|m,For my performance, I will be requiring the aid of this...|m,It's nearly time, and to gauge your excitement, I will be using this..."
    if existing_data != None:
        transitions = existing_data["signposts"]
        slide_transitions = ""
        for item in transitions:
            slide_transitions += item["position"][0] + "," + item["signpost"] + "|"
    layout = [[sg.Text("Prompt: "), sg.InputText("I'm about to do what you're all afraid of. That's right, I'm going to: <BLANK>" if existing_data == None else existing_data["title"], key="talk_title", size=(200,1))], [sg.Checkbox("Contains adult content", default=False if existing_data == None else existing_data["x"], key="x")],
    [sg.Text("Safety Answers (separate by |): "), sg.Multiline(default_text="Do absolutely nothing|Eat a snake live on camera|Downvote a post on reddit" if existing_data == None else "|".join(existing_data["safetyAnswers"]), key="safety_answers")],
    [sg.Text("Slide Transitions (separate by |, add (m,) for Middle of presentation, (e,) for End of presentation at the beginning for each transition. Slide Transitions are optional.):")],
    [sg.Multiline(default_text=slide_transitions, key="transitions", size=(200, 5))],
    [sg.Button("Make Prompt"), sg.Button("Go Back")]]
    window = sg.Window("Make a prompt" if existing_data == None else existing_data["id"], layout)
    while True:
        event, values = window.read()
        if event == sg.WIN_CLOSED:
            break
        if event == "Go Back":
            window.close()
            if existing_data == None:
                talking_points.run()
            break
        if event == "Make Prompt":
            safety_answers = values["safety_answers"].split("|")
            transitions = values["transitions"]
            transitions_list = []
            if transitions != "" and (transitions[0] == "e" or transitions[0] == "m"):
                transitions = values["transitions"].split("|")
                for item in transitions:
                    if len(item) > 2 and ("e," in item or "m," in item):
                        position = item[0]
                        signpost = item[2:] #Ignore the m, and e,
                        transitions_list.append({"position": position, "signpost": signpost})
            custom_prompt = CustomContent({"safetyAnswers": safety_answers, "signposts": transitions_list, "title": values["talk_title"], "x": values["x"]}, "JackboxTalks", "JackboxTalksTitle", values["talk_title"])
            custom_prompt.write_to_json()
            custom_prompt.save_to_custom_content()
            sg.Popup("Custom prompt created, ID: " + custom_prompt.id)
    window.close()

def talking_points_slide_transition(selection=None, existing_data=None):
    layout=[[sg.Text("Transition Text: "), sg.InputText("Of course, now I hear you ask \"Do you have any evidence?\". Well sure..." if existing_data == None else existing_data["signpost"], key="signpost", size=(100, 1))], 
    [sg.Text("Position of transition:"), sg.Listbox(("middle", "end"), size=(10,2), default_values="middle" if existing_data == None else existing_data["position"], select_mode="LISTBOX_SELECT_MODE_SINGLE", key="position")],
    [sg.Checkbox("Contains Adult Content", key="x", default=False if existing_data == None else existing_data["x"])], [sg.Button("Make Transition"), sg.Button("Go Back")]]
    window = sg.Window("Make a slide transition" if existing_data == None else existing_data["id"], layout)
    while True:
        event, values = window.read()
        if event == sg.WIN_CLOSED:
            break
        if event == "Go Back":
            window.close()
            if existing_data == None:
                talking_points.run()
        if event == "Make Transition":
            custom_transition = CustomContent({"signpost": values["signpost"], "position": values["position"][0], "x": values["x"]}, "JackboxTalks", "JackboxTalksSignpost", values["signpost"])
            custom_transition.write_to_json()
            custom_transition.save_to_custom_content()
            sg.Popup("Transition created, ID: " + custom_transition.id)
    window.close()

talking_points = SelectionWindow("Talking Points Content Selection", ["Please select the type of content", ("Picture", "Prompt", "Slide Transition"), "talking_points_content_type"], {
    "Picture": talking_points_picture,
    "Prompt": talking_points_prompt,
    "Slide Transition": talking_points_slide_transition
}, "create_content")

#Main Menu stuff
create_content = SelectionWindow("Select a game", ["Select a game.", ("Blather Round", "Devils and the Details", "Talking Points", "Quiplash 3", "Champ'd Up"), "game"],{
    "Blather Round": None,
    "Devils and the Details": None,
    "Talking Points": talking_points.run,
    "Quiplash 3": quiplash_3.run,
    "Champ'd Up": None
}, "main_window")

main_window = SelectionWindow("Select an option", ["Please select an option.", ("Create Custom Content", "View/Edit Content", "Import Content"), "option"], {
    "Create Custom Content": create_content.run,
    "View/Edit Content": edit_content,
    "Import Content": import_content
})
window_mapping = { #Used for backing out of stuff.
    "quiplash_prompt": quiplash_prompt,
    "quiplash_3": quiplash_3,
    "create_content": create_content,
    "main_window": main_window,
    "talking_points": talking_points
}
content_type_mapping = { #Used in editing content to change data.
    "Quiplash3Round1Question": round_prompt,
    "Quiplash3Round2Question": round_prompt,
    "Quiplash3FinalQuestion": round_prompt_final,
    "Quiplash3SafetyQuips": safety_quip,
    "JackboxTalksPicture": talking_points_picture,
    "JackboxTalksTitle": talking_points_prompt,
    "JackboxTalksSignpost": talking_points_slide_transition
}
main_window.run()