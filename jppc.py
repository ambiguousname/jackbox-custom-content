import PySimpleGUI as sg
import json
import os
from shutil import copyfile, rmtree

#TODO:
# Test safety quips
# Test adding and deleting specific content (Probably not gonna work right now)
# Test editing custom content
# Add other people's custom content with import
# Add Talking Points Content (Pictures, Prompts, Slide Transitions)

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
        ids.truncate()
        ids.write(json.dumps(content))
        ids.close()

    def add_custom_files(self, *args, **kwargs): #Construct the path from what we already know.
        path = ""
        if "path" in kwargs:
            path = kwargs["path"]
        else:
            path = "./" + self.values["game"] + "/content/" + self.values["content_type"] + "/" + self.id + "/"
        if os.path.exists(path):
            rmtree(path)
        if not ("delete" in kwargs and kwargs["delete"] == True):
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
        n_layout = [[sg.Text(self.layout_list[0])], [sg.Listbox(self.layout_list[1], size=(30, 10), key=self.layout_list[2])], [sg.Button('Ok'), sg.Button('Exit' if not self.previous_window != None else 'Go Back')]]
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
        window = None
                
#Stuff for file management

def view_content(selected=None):
    if os.path.exists("./custom_content.json"):
        ids = open("./custom_content.json", "r")
        content = json.load(ids)
        ids.close()
        content_str = ""
        for i in content:
            item = content[i]["values"]
            content_str += "ID: " + item["id"] + ", Game: " + item["game"] + ", Type: " + item["content_type"] + ", Description: " + item["descriptor_text"] + "; \n"
        layout = [[sg.Text(content_str)], [sg.Button("Ok")]]
        window = sg.Window("All Your Content", layout)
        while True:
            event, values = window.read()
            if event == "Ok" or event == sg.WIN_CLOSED:
                break
        window.close()
        window = None
        main_window.run()

    else:
        sg.Popup("You have no custom content.")
        main_window.run()

def edit_content(selected=None): #Selected goes unused because of how SelectWindow works.
    if os.path.exists("./custom_content.json"):
        ids = open("./custom_content.json", 'r+')
        content = json.load(ids)
        content_list = []
        for item in content:
            content_list.append(content[item]["id"] + ": " + content[item]["values"]["content_type"] + " - " + content[item]["values"]["descriptor_text"])
        layout = [[sg.Text("Choose Content to Edit/Delete:")], [sg.Listbox(content_list, key="content_selection", size=(100, 5))], [sg.Button("Edit"), sg.Button("Delete"), sg.Button("Show Folder"), sg.Button("Go Back")]]
        window = sg.Window("Choose Content to Edit/Delete", layout)
        while True:
            event, values = window.read()
            if event == sg.WIN_CLOSED:
                break
            if event == "Show Folder":
                _id = values["content_selection"][0].split(":")[0]
                existing_data = content[_id]["values"]
                path = os.path.realpath("./" + existing_data["game"] + "/content/" + existing_data["content_type"] + "/" + existing_data["id"])
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
                ids.truncate()
                if len(content.keys()) != 0:
                    ids.write(json.dumps(content))
                    ids.close()
                else:
                    ids.close()
                    os.remove("./custom_content.json")
                window.close()
                window = None
                sg.Popup("Content deleted!")
                edit_content() #To update the list of content
            if event == "Go Back":
                window.close()
                window = None
                main_window.run()
        ids.close()
        window.close()
        window = None
    else:
        sg.Popup("Sorry, no content to edit.")
        main_window.run()

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
    [sg.Text(".ogg files of you reading things (Optional):")], [sg.Text(".ogg of you saying the prompt: "), sg.InputText(key="prompt"), sg.FileBrowse()],
    [sg.Text("Add a response to specific text (Very optional, see Readme for information):")],
    [sg.InputText(key="response"), sg.FileBrowse()], 
    [sg.Text("What to filter (See Readme): "), sg.InputText(key="response-filter")],
    [sg.Text("Transcript of your response: "), sg.InputText(key="response-narration")],
    [sg.Button("Make a prompt"), sg.Button("Go Back")]]
    window = sg.Window(("Round " + selection[-1] + " Prompt") if existing_data == None else existing_data["content_type"], layout)
    while True:
        event, values = window.read()
        if event == sg.WIN_CLOSED:
            break
        if event == "Go Back":
            window.close()
            window = None
            if existing_data == None:
                quiplash_prompt.run()
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
    window = None

def round_prompt_final(selection, existing_data=None): #I'm making this separate because it's just easier to do this than to explain the last round prompt syntax.
    layout = [[sg.Text("Prompt Text: "), sg.InputText("Three things a stranger would say about <ANYPLAYER>." if existing_data == None else existing_data["prompt"], key="lastround-prompt")], [sg.Text("Safety Quip(s) (separate by |): "), sg.InputText("not|funny|didn't laugh|get|out of|my face|learn|how the safety quips|work" if existing_data == None else existing_data["safetyQuips"], key="lastround-safety-quips")],
    [sg.Checkbox("Includes Player Name", default=(True if existing_data == None else existing_data["includesPlayerName"]), key="player-name"), sg.Checkbox("Contains Adult Content", default=(False if existing_data == None else existing_data["x"]), key="x"), sg.Checkbox("Content is US specific", default=(False if existing_data == None else existing_data["us"]), key="us")],
    [sg.Text(".ogg file of you reading the prompt (Optional):")], [sg.InputText(key="prompt"), sg.FileBrowse()],
    [sg.Button("Make a prompt"), sg.Button("Go Back")]]
    window = sg.Window("Make a Quiplash 3 Final Round Prompt", layout)
    while True:
        event, values = window.read()
        if event == sg.WIN_CLOSED:
            break
        if event == "Go Back":
            window.close()
            window = None
            if existing_data == None:
                quiplash_prompt.run()
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
    window = None

def safety_quip(selection, existing_data=None):
    layout = [[sg.Text("Safety Quip Text (Should be generic): "), sg.InputText("" if existing_data == None else existing_data["value"], key="safety-quip")], [sg.Button("Make Quip"), sg.Button("Go Back")]]
    window = sg.Window("Make a Safety Quip", layout)
    while True:
        event, values = window.read()
        if event == "Make Quip":
            quip = CustomContent({"value": values["safety-quip"]}, "Quiplash3", "Quiplash3SafetyQuips", values["safety-quip"], None if existing_data == None else existing_data["id"])
            quip.write_to_json()
            sg.Popup("Safety Quip Created. ID: " + quip.id)
        if event == "Go Back":
            window.close()
            window = None
            if existing_data == None:
                quiplash_3.run()

quiplash_prompt = SelectionWindow("Choose a Round", ["Choose a round.", ("Round 1", "Round 2", "Final Round"), "round-number"], {
    "Round 1": round_prompt,
    "Round 2": round_prompt,
    "Final Round": round_prompt_final
}, "quiplash_3")

quiplash_3 = SelectionWindow("Quiplash 3 Content Selection", ["Please select the type of content", ("Prompt", "Safety Quip"), "content_type"], {
    "Prompt": quiplash_prompt.run,
    "Safety Quip": safety_quip
}, "create_content")

#Main Menu stuff
create_content = SelectionWindow("Select a game", ["Select a game.", ("Quiplash 3"), "game"],{
    "Blather Round": None,
    "Devils and the Details": None,
    "Talking Points": None,
    "Quiplash 3": quiplash_3.run,
    "Champ'd Up": None
}, "main_window")

main_window = SelectionWindow("Select an option", ["Please select an option.", ("View My Custom Content", "Create Custom Content", "Edit Content"), "option"], {
    "Create Custom Content": create_content.run,
    "Edit Content": edit_content,
    "Import Content": None,
    "View My Custom Content": view_content
})
window_mapping = { #Used for backing out of stuff.
    "quiplash_prompt": quiplash_prompt,
    "quiplash_3": quiplash_3,
    "create_content": create_content,
    "main_window": main_window
}
content_type_mapping = { #Used in editing content to change data.
    "Quiplash3Round1Question": round_prompt,
    "Quiplash3Round2Question": round_prompt,
    "Quiplash3FinalQuestion": round_prompt_final,
    "Quiplash3SafetyQuips": safety_quip
}
main_window.run()