import PySimpleGUI as sg
import json
import os
from shutil import copyfile, rmtree

#TODO:
# Add warning if the .EXE file isn't in the ./games/ folder.
# Test Quiplash 3 Custom Responses
# Test Champ'd Up
# Test Talking Points
# Test Blather 'Round

def id_gen(values): #id_gen needs a values dict to work with
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
    values.update({"id": _id}) #Need to store the id twice so that things can work. The .jet files need a reference to the id.
    id_dict[_id] = {"id": _id, "values": values}
    ids.seek(0)
    ids.truncate()
    new_json = json.dumps(id_dict)
    ids.write(new_json)
    ids.close()
    return _id

class CustomContentWindow(object):

    def __init__(self, *args, **kwargs):
        self.window_layout = args[3]
        self.game = args[0]
        self.content_type = args[1]
        self.descriptor_text_name = args[2]

    def create_content(self, values, _id=None, window_suppress=False):
        new_content = CustomContent(values, self.game, self.content_type, values[self.descriptor_text_name], id=None if _id == None else _id)
        new_content.save_to_custom_content()
        for content in self.window_layout["content_list"]:
            if content["type"] == "json":
                new_content.write_to_json(None if not "path" in content else content["path"], False if not "delete" in content else content["delete"])
            if content["type"] == "files":
                kwargs = content["files"]["kwargs"]
                args = content["files"]["args"]
                new_content.add_custom_files(*args, **kwargs)
            if content["type"] == "CustomData":
                data = content["func"](new_content.values)
                kwargs = content["kwargs"]
                new_content.add_custom_files(data, **kwargs)
        if window_suppress == False:
            sg.Popup("Content Created, ID: " + new_content.id)
        return new_content.id

    def create_window(self, *args, **kwargs):
        existing_data = None
        if "existing_data" in kwargs and kwargs["existing_data"] != None:
            existing_data = kwargs["existing_data"]
            if "import_filter" in self.window_layout:
                existing_data = self.window_layout["import_filter"](kwargs["existing_data"])
        if self.window_layout and "layout_list" in self.window_layout:
            self.file_browse_keys = []
            layout = []
            for item in self.window_layout["layout_list"]:
                layout_item = []
                if "text" in item:
                    layout_item.append(sg.Text(item["text"]))
                if "input" in item:
                    for index, input_type in enumerate(item["input"]):
                        new_kwargs = {}
                        if "kwargs" in input_type:
                            new_kwargs = input_type["kwargs"].copy()
                            for new_kwarg in new_kwargs:
                                if (type(new_kwargs[new_kwarg]) == str and "existing_data" in new_kwargs[new_kwarg]) or input_type["type"] == sg.Checkbox:
                                    if existing_data != None:
                                        if type(new_kwargs[new_kwarg]) == str and len(new_kwargs[new_kwarg].split("|")) > 1:
                                            new_kwargs[new_kwarg] = existing_data[new_kwargs[new_kwarg].split("|")[1]]
                                        else:
                                            new_kwargs[new_kwarg] = existing_data[input_type["param_name"]]
                                    else:
                                        new_kwargs[new_kwarg] = new_kwargs["regular_default"]
                        if "regular_default" in new_kwargs:
                            new_kwargs.pop("regular_default")
                        if input_type["type"] == sg.FileBrowse:
                            self.file_browse_keys.append(item["input"][index - 1]["param_name"])
                        exclude = [sg.FileBrowse, sg.Checkbox]
                        new_input = input_type["type"](input_type["default_value"] if existing_data == None or input_type["type"] in exclude else existing_data[input_type["param_name"]], key=input_type["param_name"], **new_kwargs)
                        layout_item.append(new_input)
                layout.append(layout_item)
            layout.append([sg.Button("Ok") if existing_data == None else sg.Button("Edit"), sg.Button("Make New Content") if existing_data != None else sg.Text(), sg.Button("Go Back") if existing_data == None or ("Go Back" in existing_data and existing_data["Go Back"] == True) else sg.Button("Exit")])
            window = sg.Window(self.content_type if existing_data == None else "Editing " + existing_data["id"], layout)
            while True:
                event, values = window.read()
                if event == sg.WIN_CLOSED or event == "Exit":
                    break
                if event == "Ok" or event == "Edit" or event == "Make New Content":
                    new_values = values
                    for value in new_values:
                        if type(new_values[value]) == str:
                            new_values[value] = new_values[value].replace("\n", "")
                    if "filter" in self.window_layout:
                        new_values = self.window_layout["filter"](new_values)
                    _id = None
                    for key in self.file_browse_keys:
                        curr_path = new_values[key].split("/")
                        new_path = ""
                        file_path = "./external content/"
                        looking_for_external = True
                        for directory in curr_path:
                            if looking_for_external == True:
                                new_path += directory + "/"
                                if directory == "external content":
                                    looking_for_external = False
                            else:
                                file_path += directory + "/"
                        if "external content" in curr_path and os.path.realpath("./external content/") == os.path.realpath(new_path):
                            new_values[key] = file_path
                    if existing_data != None and event != "Make New Content":
                        _id = existing_data["id"]
                    new_content = self.create_content(new_values, _id)
                    window.close()
                    go_back = True
                    if existing_data != None and not "Go Back" in existing_data:
                        go_back = False
                    elif existing_data != None and "Go Back" in existing_data:
                        go_back = existing_data["Go Back"]
                    existing_data = new_values
                    existing_data["Go Back"] = go_back
                    existing_data["id"] = new_content
                    self.create_window(existing_data=existing_data)
                if event == "Go Back":
                    window.close()
                    window_mapping[self.window_layout["previous_window"]].run()
            window.close()
        else:
            raise Exception("Did not Instantiate CustomContentWindow with 'window_layout' kwarg.")

class CustomContent(object):
    def __init__(self, *args, **kwargs): #values, game, content_type, descriptor_text, _id=None
        self.values = {"game": args[1], "content_type": args[2], "descriptor_text": args[3]} #We stores these in .values because .values is written to custom_content.json for content editing.
        self.values.update(args[0])
        if "id" in kwargs and type(kwargs["id"]) == str and kwargs["id"].isnumeric():
            self.id = kwargs["id"]
        else:
            self.id = id_gen(self.values)
        self.values.update({"id": self.id})

    def write_to_json(self, p=None, delete=False, data=None):
        path = "" 
        if p:
            path = p
        else:
            path = "./" + self.values["game"] + "/content/" + self.values["content_type"] + ".jet"
        if data != None:
            if os.path.exists(path):
                os.remove(path)
            if delete == False:
                custom_data = open(path, "w")
                custom_data.write(json.dumps(data.values))
                custom_data.close()
        elif os.path.exists(path): #Are we making a new .JSON file, or are we appending to an existing .JSON file?
            jf = open(path, "r", encoding="utf-8")
            json_file = json.load(jf)
            if delete == True:
                if self.values in json_file["content"]:
                    json_file["content"].remove(self.values)
                else:
                    sg.Popup("Sorry, that content doesn't appear to exist in the game's files. Try saving custom_content.json somewhere else, using the Reset All Custom Content feature, and then importing custom_content.json back in.")
            else:
                #Work backwards. It's faster that way.
                for i in range(len(json_file["content"]) - 1, -1, -1):
                    cont = json_file["content"][i]
                    if cont["id"] == self.id:
                        json_file["content"].pop(i)
                        break
                    if int(cont["id"]) < 100000:
                        break
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
    
    def save_to_custom_content(self): #Save to custom_content.json file which keeps track of everything. Call this first before adding content. We have values as an argument to pass any additional values.
        ids = open("./custom_content.json", "r+")
        content = json.load(ids)
        if self.id in content:
            content[self.id].update({"id": self.id, "values": self.values})
        else:
            content[self.id] = {"id": self.id, "values": self.values}
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
        if os.path.exists(path) and not ("adding_other_files" in kwargs and kwargs["adding_other_files"] == True): #If there's a folder here, but we're not selecting a custom path
            rmtree(path)
        if not ("delete" in kwargs and kwargs["delete"] == True):
            if not (os.path.exists(path)):
                os.mkdir(path)
            for file in args:
                if type(file) == dict and "path" in file: #If we're just copying a file
                    file_path = file["path"]
                    name = file["name"]
                    if name == "id":
                        name = self.id + file["extension"]
                    if file_path == "param_name":
                        file_path = self.values[file["param_name"]]
                    if(os.path.exists(file_path)): #Only add this if the file's path exists.
                        file_path = os.path.realpath(file_path)
                        copyfile(file_path, path + "/" + name) #From shutil
                    elif file_path != "./" and file_path != "":
                        sg.Popup("The file: " + file_path + " does not exist. Custom content added anyway (please edit content #: " + self.id + " in the view/edit content menu.")
                else: #If we're going to be writing a custom file from like a .JSON or whatever.
                    if isinstance(file, CustomContent):
                        if os.path.exists(path + "data.jet"):
                            os.remove(path + "data.jet")
                        file.write_to_json(path + "data.jet", False, args[0])
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
        n_layout = [[sg.Text(self.layout_list[0])], [sg.Listbox(self.layout_list[1], size=(30, 10), select_mode=sg.LISTBOX_SELECT_MODE_BROWSE, key=self.layout_list[2])], [sg.Button('Ok'), sg.Button('Exit' if not self.previous_window != None else 'Go Back')]]
        window = sg.Window(self.title, n_layout)
        while True:
            event, values = window.read()
            if event == sg.WIN_CLOSED or event == "Exit":
                break
            if event == "Ok":
                if len(values[self.list_key]) == 1 and values[self.list_key][0] in self.selector:
                    window.close()
                    func = self.selector.get(values[self.list_key][0])
                    func(values[self.list_key][0]) #What we need the "inputs" argument for. 
                    break
                elif "all" in self.selector and len(values[self.list_key]) == 1:
                    window.close()
                    func = self.selector.get("all")
                    func(values[self.list_key][0])
                    break
            if event == "Go Back" and self.previous_window:
                window.close()
                if hasattr(window_mapping[self.previous_window], "run"):
                    window_mapping[self.previous_window].run()
                else:
                    window_mapping[self.previous_window]()
                break
        window.close()
                
#Stuff for file management

def edit_content_window(selected=None): #Selected goes unused because of how SelectWindow works.
    previous_window = content_type_mapping
    if len(selected.split(" ")) >= 3 and selected.split(" ")[1] == "All" and selected.split(" ")[2] == "Content":
        previous_window = selected.split(" ")[0]
    elif selected != "All Games":
        for game in content_type_mapping:
            for content_type in content_type_mapping[game]:
                if content_type == selected:
                    previous_window = game
                    break
    if os.path.exists("./custom_content.json"):
        ids = open("./custom_content.json", 'r+')
        content = json.load(ids)
        content_list = []
        for item in content:
            if content[item]["values"]["content_type"] == selected or selected == "All Games" or (len(selected.split(" ")) >= 3 and selected.split(" ")[1] == "All" and selected.split(" ")[2] == "Content" and content[item]["values"]["game"] == selected.split(" ")[0]):
                content_list.append(content[item]["id"] + ": " + content[item]["values"]["content_type"] + " - " + content[item]["values"]["descriptor_text"])
        layout = [[sg.Text("Choose Content to Edit/Delete:")], [sg.Listbox(content_list, key="content_selection", size=(100, 25), select_mode=sg.LISTBOX_SELECT_MODE_EXTENDED)], [sg.Button("Edit"), sg.Button("Delete"), sg.Button("Show Folder"), sg.Button("Go Back")]]
        window = sg.Window("Choose Content to Edit/Delete", layout)
        while True:
            event, values = window.read()
            if event == sg.WIN_CLOSED:
                break
            if event == "Show Folder":
                _id = values["content_selection"][0].split(":")[0]
                existing_data = content[_id]["values"]
                path = os.path.realpath("./" + existing_data["game"] + "/content/" + existing_data["content_type"] + "/" + existing_data["id"])
                if "custom_file_path" in existing_data:
                    path = os.path.realpath(existing_data["custom_file_path"])
                if (os.path.exists(path)):
                    if(os.path.isfile(path)):
                        path = path + "/../"
                    os.startfile(path)
                else:
                    sg.Popup("This content cannot be found in an easily accessible folder.")
            if event == "Edit":
                for item in values["content_selection"]:
                    _id = item.split(":")[0]
                    existing_data = content[_id]["values"]
                    content_type_mapping[existing_data["game"]][existing_data["content_type"]].create_window(existing_data=existing_data)
                window.close()
                ids.close()
                edit_content_window(selected)
                break
            if event == "Delete":
                for item in values["content_selection"]:
                    _id = item.split(":")[0]
                    custom_content = CustomContent(content[_id]["values"], content[_id]["values"]["game"], content[_id]["values"]["content_type"], content[_id]["values"]["content_type"], content[_id]["values"]["descriptor_text"], id=_id) #Setting None because values already has the game, type, and descriptor_text.
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
                ids.close()
                sg.Popup("Content deleted!")
                edit_content_window(selected) #To update the list of content
                break
            if event == "Go Back":
                ids.close()
                window.close()
                if selected == "All Games":
                    edit_content()
                else:
                    game_content_select(previous_window)
                break
        ids.close()
        window.close()
    else:
        sg.Popup("Sorry, no content to edit.")
        if selected == "All Games":
            edit_content()
        else:
            game_content_select(previous_window)

def game_content_select(selected=None):
    layout_list = tuple(content_type_mapping[selected].keys()) + tuple([selected + " All Content"])
    select_window = SelectionWindow("Select content type to view/edit for " + selected + ".", ["Select content type to view/edit for " + selected + ".", layout_list, "edit_content_type_selector"], {
        "all": edit_content_window
    }, "edit_content")
    select_window.run()
        
def edit_content(selected=None):
    layout_list = tuple(content_type_mapping.keys()) + tuple(["All Games"])
    select_window = SelectionWindow("Select a game to view/edit content for.", ["Select game to view/edit content for.", layout_list, "edit_content_selector"], {
        "all": game_content_select,
        "All Games": edit_content_window
    }, "main_window")
    select_window.run()

def import_content(path="./custom_content.json"):
    if os.path.exists(path):
        new_ids = open(path, "r")
        new_content = json.load(new_ids)
        new_ids.close()
        is_copying = False
        if not os.path.exists("./custom_content.json"):
            copyfile(path, "./custom_content.json")
            is_copying = True
        if os.path.realpath(path) == os.path.realpath("./custom_content.json"):
            is_copying = True
        ids = open("./custom_content.json", "r+")
        content = json.load(ids)
        content_keys = list(content.keys())
        content_keys.sort()
        latest_id = 100000 if is_copying == True else int(content_keys[-1]) + 1
        for i in new_content:
            content_id = str(latest_id + int(i) - 100000)
            n_c = new_content[i]
            content[content_id] = n_c
            n_c["id"] = content_id
            n_c["values"]["id"] = content_id
            content_type_mapping[n_c["values"]["game"]][n_c["values"]["content_type"]].create_content(n_c["values"], content_id, True) #Will bug you with popups rn.
        ids.seek(0)
        ids.truncate()
        ids.write(json.dumps(content))
        ids.close()

def import_content_window(selected=None):
    layout = [[sg.Text("To share content for import, share custom_content.json (from the same folder as Jackbox Party Pack Custom.exe). NOTE: See the readme for importing files like .OGGs or .JPGs.")],
    [sg.Text("If you've restored your game's files to their original condition, select custom_content.json to reimport.")],
    [sg.Text("If that file has been shared with you, select it here.: "), sg.InputText(key="custom-files"), sg.FileBrowse(file_types=((".JSON", "*.json"), ("ALL Types", "*.*")))], [sg.Button("Import"), sg.Button("Go Back")]]
    window = sg.Window("Select File to Import", layout)
    while True:
        event, values = window.read()
        if event == sg.WIN_CLOSED:
            break
        if event == "Import":
            if os.path.exists(values["custom-files"]) and os.path.splitext(values["custom-files"])[1].lower() == ".json":
                import_content(values["custom-files"])
                sg.Popup("Custom content imported. View the files in the edit menu.")
            else:
                sg.Popup("That file doesn't exist, or it isn't a .json file.")
        if event == "Go Back":
            window.close()
            main_window.run()
            break
    window.close()

def game_content_del(game, content_type):
    content = content_type_mapping[game][content_type].window_layout["content_list"]
    for item in content:
        file_type = item["type"]
        if file_type == "json": #WE ONLY NEED THE .JSON FILE TO DELETE, IDIOT!
            path = "./" + game + "/content/" + content_type + ".jet"
            jet_file = open(path, "r", encoding="utf-8")
            json_file = json.load(jet_file)
            new_content_list = []
            for content_piece in json_file["content"]:
                if int(content_piece["id"]) >= 100000:
                    new_content_list.append(content_piece)
            json_file["content"] = new_content_list
            jet_file.close()
            jet_file = open(path, "w")
            jet_file.truncate()
            jet_file.write(json.dumps(json_file))
            jet_file.close()

def del_by_content(game):
    layout = [[sg.Text("Again, are you absolutely sure you want to do this?")],
    [sg.Text("Please select the content type(s) you'd like to remove: "), sg.Listbox(([item for item in content_type_mapping[game]]), size=(50, 4), key="content_choice", select_mode=sg.LISTBOX_SELECT_MODE_MULTIPLE)], [sg.Checkbox("I am absolutely sure I want to do this. Please delete the non-custom content types I've selected.", key="sure")],
    [sg.Button("Ok"), sg.Button("Go Back")]]
    window = sg.Window("Delete content types", layout)
    while True:
        event, values = window.read()
        if event == sg.WIN_CLOSED:
            break
        if event == "Go Back":
            window.close()
            del_all_else()
            break
        if event == "Ok":
            if values["sure"] == True:
                for content_type in values["content_choice"]:
                    game_content_del(game, content_type)
            sg.Popup("Non-Custom Content deleted for: " + str(values["content_choice"]))

def del_all_else(selected=None):
    layout = [[sg.Text("Are you absolutely sure you want to do this?")], [sg.Text("This option will effectively delete all the game's content files so that you can only play with your own custom content. Please make sure you have backups.")],
    [sg.Text("Please select the game whose content you'd like to remove: "), sg.Listbox(([item for item in content_type_mapping]), size=(50, 4), key="game_choice", select_mode=sg.LISTBOX_SELECT_MODE_BROWSE)], [sg.Checkbox("I am absolutely sure I want to do this. Please delete all non-custom content for the game I have selected.", key="sure")], [sg.Button("Ok"), sg.Button("Remove Non-Custom Content By Type"), sg.Button("Go Back")]]
    window = sg.Window("Delete non-custom content", layout)
    while True:
        event, values = window.read()
        if event == sg.WIN_CLOSED:
            break
        if event == "Go Back":
            window.close()
            main_window.run()
            break
        if event == "Ok":
            if values["sure"] == True:
                for game in values["game_choice"]:
                    for content_type in content_type_mapping[game]:
                        game_content_del(game, content_type)
                sg.Popup("Non-Custom Content deleted for: " + str(values["game_choice"]))
        if event == "Remove Non-Custom Content By Type":
            window.close()
            for game in values["game_choice"]:
                del_by_content(game)
    window.close()


#Stuff for Quiplash 3

def create_quiplash_data_jet(values):
    data = CustomData()
    data.add_data("B", "true" if values["response_filter"] != "" else "false", "HasJokeAudio") 
    data.add_data("S", values["response_filter"], "Keywords")
    data.add_data("A", "response", "KeywordResponseAudio") #Included even though there might not be response audio
    data.add_data("S", values["response_transcript"], "KeywordResponseText")
    data.add_data("B", "true" if values["prompt"] != "" else "false", "HasPromptAudio")
    data.add_data("A", "prompt", "PromptAudio") #I think this is asking for the file name of the audio. I think I can leave this in if the audio doesn't exist, because some prompts don't have response audio, but we include the above line. 
    data.add_data("S", values["prompt"], "PromptText")
    data.add_data("S", "|".join(values["safetyQuips"]), "SafetyQuips")
    return data

def round_filter(values):
    new_values = values
    new_values["safetyQuips"] = values["safetyQuips"].split("|")
    return new_values

def round_import(values):
    new_values = values
    new_values["safetyQuips"] = "|".join(values["safetyQuips"])
    return new_values

round_prompt_layout = {
    "previous_window": "quiplash_prompt",
    "layout_list": [{"text": "Prompt Text: ", "input": [
        {
        "type": sg.InputText,
        "default_value": "Hey, <ANYPLAYER> needs to <BLANK>.",
        "param_name": "prompt",
        "kwargs": {"size": (50, 1)}
        }
    ]}, {"text": "Safety Quips (separate by |):", "input": [
        {
            "type": sg.Multiline,
            "default_value": "learn how the prompt system works|learn how safety quips work|eat all my garbage",
            "param_name": "safetyQuips",
            "kwargs": {"size": (50, 1)}
        }
    ]}, {"input": [
        {
            "type": sg.Checkbox,
            "default_value": "Includes Player Name",
            "kwargs": {"default": "existing_data", "regular_default": True},
            "param_name": "includesPlayerName"
        },
        {
            "type": sg.Checkbox,
            "default_value": "Contains Adult Content",
            "kwargs": {"default": "existing_data", "regular_default": False},
            "param_name": "x"
        }, {
            "type": sg.Checkbox,
            "default_value": "Content is US-Specific",
            "kwargs": {"default": "existing_data", "regular_default": False},
            "param_name": "us"
        }
    ]}, {"text": ".ogg files of you reading the prompt (Optional):", "input": [
        {
            "type": sg.InputText,
            "default_value": "",
            "param_name": "prompt_sound"
        }, {
            "type": sg.FileBrowse,
            "default_value": "Browse",
            "kwargs": {
                "file_types": [(".OGG", "*.ogg"), ("ALL Files", "*.*")]
            },
            "param_name": "prompt_file_browse"}
    ]}, {"text": "Add a response to specific text (Very optional, see Readme for information):", "input": [
        {
            "type": sg.InputText,
            "default_value": "",
            "param_name": "response_sound"
        }, {
            "type": sg.FileBrowse,
            "default_value": "Browse",
            "kwargs": {
                "file_types": [(".OGG", "*.ogg"), ("ALL Files", "*.*")]
            },
            "param_name": "response_file_browse"
        }
    ]}, {"text": "What to filter (See Readme): ", "input": [
        {
            "type": sg.InputText,
            "default_value": "",
            "param_name": "response_filter"
        }
    ]}, {"text": "Transcript of your response: ", "input": [
        {
            "type": sg.InputText,
            "default_value": "",
            "param_name": "response_transcript"
        }
    ]}],
    "content_list": [
        {"type": "json"}, #Write to master .JET file
        {"type": "CustomData", "func": create_quiplash_data_jet, "kwargs": {}},
        {"type": "files", "files": {
            "args": [{"path": "param_name", "param_name": "prompt_sound", "name": "prompt.ogg"}, {"path": "param_name", "param_name": "response_sound", "name": "response.ogg"}],
            "kwargs": {"adding_other_files": True}
        }}
    ],
    "filter": round_filter,
    "import_filter": round_import
}

round_prompt_1 = CustomContentWindow("Quiplash3", "Quiplash3Round1Question", "prompt", round_prompt_layout)

round_prompt_2 = CustomContentWindow("Quiplash3", "Quiplash3Round2Question", "prompt", round_prompt_layout)

def round_final_filter(values):
    new_values = values
    formatted_quips = []
    safety_quip = new_values["safetyQuips"].split("|")
    for i in range(0, len(safety_quip), 3):
        if not (i + 3 > len(safety_quip)):
            formatted_quips.append(safety_quip[i] + "|" + safety_quip[i + 1] + "|" + safety_quip[i + 2])
    new_values["safetyQuips"] = formatted_quips
    new_values["response_filter"] = ""
    new_values["response_transcript"] = ""
    return new_values

def round_final_import(values):
    new_values = values
    new_values["safetyQuips"] = "|".join(values["safetyQuips"])
    return new_values

round_prompt_final = CustomContentWindow("Quiplash3", "Quiplash3FinalQuestion", "prompt", {
    "previous_window": "quiplash_prompt",
    "layout_list": [{"text": "Prompt Text: ", "input": [
        {
            "type": sg.InputText,
            "default_value": "<ANYPLAYER>'s three favorite words.",
            "param_name": "prompt"
        }
    ]}, {"text": "Safety Quip(s) (separate by |):", "input": [
        {
            "type": sg.Multiline,
            "default_value": "learning|safety|quips|wait|sorry|what|what|is|love",
            "param_name": "safetyQuips"
        }
    ]}, {"input": [
        {
            "type": sg.Checkbox,
            "default_value": "Includes Player Name",
            "kwargs": {"default": "existing_data", "regular_default": True},
            "param_name": "includesPlayerName"
        }, {
            "type": sg.Checkbox,
            "default_value": "Contains Adult Content",
            "kwargs": {"default": "existing_data", "regular_default": False},
            "param_name": "x"
        }, {
            "type": sg.Checkbox,
            "default_value": "Content is US-Specific",
            "kwargs": {"default": "existing_data", "regular_default": False},
            "param_name": "us"
        }
    ]}, {"text": ".ogg file of you reading the prompt (Optional):", "input": [
        {
            "type": sg.InputText,
            "default_value": "",
            "param_name": "prompt_sound"
        }, {
            "type": sg.FileBrowse,
            "default_value": "Browse",
            "kwargs": {
                "file_types": [(".OGG", "*.ogg"), ("ALL Files", "*.*")]
            },
            "param_name": "prompt_file_browse"
        }
    ]}],
    "content_list": [
        {"type": "json"},
        {"type": "CustomData", "func": create_quiplash_data_jet, "kwargs": {}},
        {"type": "files", "files": {
            "args": [{"path": "param_name", "param_name": "prompt_sound", "name": "prompt.ogg"}],
            "kwargs": {"adding_other_files": True}
        }}
    ],
    "filter": round_final_filter,
    "import_filter": round_final_import
})

safety_quip = CustomContentWindow("Quiplash3", "Quiplash3SafetyQuips", "value", {
    "previous_window": "quiplash_3",
    "layout_list": [{"text": "Safety Quip Text (Should be generic): ", "input": [
        {
            "type": sg.InputText,
            "default_value": "",
            "param_name": "value"
        }
    ]}],
    "content_list": [{"type": "json"}]
})

quiplash_prompt = SelectionWindow("Choose a Round", ["Choose a round.", ("Round 1", "Round 2", "Final Round"), "quiplash3_round_number"], {
    "Round 1": round_prompt_1.create_window,
    "Round 2": round_prompt_2.create_window,
    "Final Round": round_prompt_final.create_window
}, "quiplash_3")

quiplash_3 = SelectionWindow("Quiplash 3 Content Selection", ["Please select the type of content", ("Prompt", "Safety Quip"), "quiplash3_content_type"], {
    "Prompt": quiplash_prompt.run,
    "Safety Quip": safety_quip.create_window
}, "create_content")

#Stuff for Talking Points

def talking_points_picture_filter(values):
    new_values = values
    if new_values["low_res_path"] == "":
        new_values["low_res_path"] = new_values["file_path"]
    new_values["custom_file_path"] = "./JackboxTalks/content/JackboxTalksPicture/"
    return new_values

talking_points_picture = CustomContentWindow("JackboxTalks", "JackboxTalksPicture", "name", {
    "previous_window": "talking_points",
    "layout_list": [{"text": "Choose a .JPG file (will show up on your mobile device as a black photo, but it will appear in the game itself): ", "input": [
        {
            "type": sg.InputText,
            "default_value": "",
            "param_name": "file_path"
        }, {
            "type": sg.FileBrowse,
            "default_value": "Browse",
            "param_name": "photo_file_browse",
            "kwargs": {
                "file_types": [(".JPG", "*.jpg"), ("ALL Files", "*.*")]
            }
        }
    ]}, {"text": "Low Res .JPG (recommended, will use higher-res picture if not given): ", "input": [
        {
            "type": sg.InputText,
            "default_value": "",
            "param_name": "low_res_path"
        }, {
            "type": sg.FileBrowse,
            "default_value": "Browse",
            "param_name": "low_res_file_browse",
            "kwargs": {
                "file_types": [(".JPG", "*.jpg"), ("ALL Files", "*.*")]
            }
        }
    ]}, {"text": "Description of the Picture (required): ", "input": [
        {
            "type": sg.InputText,
            "default_value": "",
            "param_name": "name"
        }
    ]}, {"input": [
        {
            "type": sg.Checkbox,
            "default_value": "Picture contains adult content",
            "kwargs": {"default": "existing_data", "regular_default": False},
            "param_name": "x"
        }
    ]}],
    "content_list": [
        {"type": "json"},
        {"type": "files", "files": {
            "args": [{"path": "param_name", "param_name": "file_path", "name": "id", "extension": ".jpg"}],
            "kwargs": {"path": "./JackboxTalks/content/JackboxTalksPicture", "adding_other_files": True}
        }},
        {"type": "files", "files": {
            "args": [{"path": "param_name", "param_name": "low_res_path", "name": "id", "extension": ".jpg"}],
            "kwargs": {"path": "./JackboxTalks/content/JackboxTalksPictureLow", "adding_other_files": True}
        }}
    ],
    "filter": talking_points_picture_filter
})

def talking_points_prompt_import(values):
    new_values = values
    slide_transitions = ""
    transitions = values["signposts"]
    for index, item in enumerate(transitions):
        slide_transitions += item["position"][0] + "," + item["signpost"] + ("|" if index < len(transitions) - 1 else "")
    new_values["signposts"] = slide_transitions
    new_values["safetyAnswers"] = "|".join(values["safetyAnswers"])
    return new_values

def talking_points_prompt_filter(values):
    new_values = values
    transitions = values["signposts"]
    transitions_list = []
    if transitions != "" and (transitions[0] == "e" or transitions[0] == "m"):
        transitions = values["signposts"].split("|")
        for item in transitions:
            if len(item) > 2 and ("e," in item or "m," in item):
                position = item[0]
                signpost = item[2:] #Ignore the m, and e,
                transitions_list.append({"position": "end" if position == "e" else "middle", "signpost": signpost})
    new_values["signposts"] = transitions_list
    new_values["safetyAnswers"] = values["safetyAnswers"].split("|")
    return new_values

talking_points_prompt = CustomContentWindow("JackboxTalks", "JackboxTalksTitle", "title", {
    "previous_window": "talking_points",
    "layout_list": [{"text": "Prompt: ", "input": [
        {
            "type": sg.InputText,
            "default_value": "I'm about to do what you're all afraid of. That's right, I'm going to: <BLANK>",
            "param_name": "title",
            "kwargs": {"size": (75, 1)}
        }
    ]}, {"text": "Safety Answers (separate by |): ", "input": [
        {
            "type": sg.Multiline,
            "default_value": "Do absolutely nothing|Eat a snake live on camera|Downvote a post on reddit",
            "param_name": "safetyAnswers"
        }
    ]}, {"text": "Slide Transitions (separate by |, add (m,) for Middle of presentation, (e,) for End of presentation at the beginning for each transition. Slide transitions are optional.):"}, {"input": [
        {
            "type": sg.Multiline,
            "default_value": "m,For those of you questioning my reasons, I was motivated by this...|m,For those of you who object, here's why you're all powerless to stop me...|m,If you're concerned about permissions, I have all the power I need from this...|e,Now for the Finale: What you're about to see next will ultimately prove my superiority...|m,What I'm about to say is actually banned in about 20 countries, so pay close attention...|e,For those of you at home, imitate exactly what you're about to hear and see...|e,Now it's flex time, and I'm going to flex with this...|e,I have no words for what you're about to witness, only vague and confusing noises/hand movements...|m,For this amazing feat, I will make use of this as a centerpiece...|m,For my performance, I will be requiring the aid of this...|m,It's nearly time, and to gauge your excitement, I will be using this...",
            "kwargs": {"size": (100, 5)},
            "param_name": "signposts"
        }
    ]}, {"input": [
        {
            "type": sg.Checkbox,
            "default_value": "Contains adult content",
            "kwargs": {"default": "existing_data", "regular_default": False},
            "param_name": "x"
        }
    ]}],
    "content_list": [
        {"type": "json"}
    ],
    "filter": talking_points_prompt_filter,
    "import_filter": talking_points_prompt_import
})

def talking_points_slide_transition_filter(values):
    new_values = values
    new_values["position"] = values["position"][0]
    new_values["select_position"] = [values["position"]]
    return new_values

def talking_points_slide_transition_import(values):
    new_values = values
    new_values["select_position"] = [values["position"]]
    new_values["position"] = ("middle", "end")
    return new_values

talking_points_slide_transition = CustomContentWindow("JackboxTalks", "JackboxTalksSignpost", "signpost", {
    "previous_window": "talking_points",
    "layout_list": [{"text": "Transition Text: ", "input": [
        {
            "type": sg.InputText,
            "default_value": "Of course, now I hear you ask \"Do you have any evidence?\" Well sure...",
            "param_name": "signpost",
            "kwargs": {"size": (100, 1)}
        }
    ]}, {"text": "Position of transition:", "input": [
        {
            "type": sg.Listbox,
            "default_value": ("middle", "end"),
            "kwargs": {"default_values": "existing_data|select_position", "size": (20, 2), "regular_default": "middle"},
            "param_name": "position",
            "position": "middle"
        }
    ]}, {"input": [
        {
            "type": sg.Checkbox,
            "default_value": "Contains Adult Content",
            "param_name": "x",
            "kwargs": {"default": "existing_data", "regular_default": False}
        }
    ]}],
    "content_list": [
        {"type": "json"}
    ],
    "filter": talking_points_slide_transition_filter,
    "import_filter": talking_points_slide_transition_import
})

talking_points = SelectionWindow("Talking Points Content Selection", ["Please select the type of content", ("Picture", "Prompt", "Slide Transition"), "talking_points_content_type"], {
    "Picture": talking_points_picture.create_window,
    "Prompt": talking_points_prompt.create_window,
    "Slide Transition": talking_points_slide_transition.create_window
}, "create_content")

# Champ'd Up Stuff

def champd_up_data_jet(values):
    data = CustomData()
    data.add_data("B", "true" if values["contest_path"] != "" else "false", "HasContestAudio") 
    data.add_data("A", "contest", "ContestAudio")
    data.add_data("S", values["contest"], "ContestText") #Included even though there might not be response audio
    data.add_data("S", values["gameText"], "GameText")
    data.add_data("B", "true" if values["response_path"] != "" else "false", "HasResponseAudio")
    data.add_data("A", "response", "ResponseAudio") #I think this is asking for the file name of the audio. I think I can leave this in if the audio doesn't exist, because some prompts don't have response audio, but we include the above line. 
    data.add_data("S", values["response_transcript"], "ResponseText")
    return data

champd_up_round_layout = {
    "previous_window": "champd_up",
    "layout_list": [{"text": "Contest Name: ", "input": [
        {
            "type": sg.InputText,
            "default_value": "The Champion of <ANYPLAYER>'s Nightmares",
            "param_name": "contest"
        }
    ]}, {"text": "A shorter description (that doesn't include the word \"Champion\"): ", "input": [
        {
            "type": sg.InputText,
            "default_value": "<ANYPLAYER>'s Nightmares",
            "param_name": "gameText"
        }
    ]}, {"text": ".OGG file of you reading the contest name (recommended)", "input": [
        {
            "type": sg.InputText,
            "default_value": "",
            "param_name": "contest_path"
        }, {
            "type": sg.FileBrowse,
            "default_value": "Browse",
            "param_name": "contest_browse",
            "kwargs": {
                "file_types": [(".OGG", "*.ogg"), ("ALL Files", "*.*")]
            }
        }
    ]}, {"text": ".OGG file of you reacting to the prompt (something like 'Oooh! Scary!') (recommended)", "input": [
        {
            "type": sg.InputText,
            "default_value": "",
            "param_name": "response_path"
        }, {
            "type": sg.FileBrowse,
            "default_value": "Browse",
            "param_name": "response_browse",
            "kwargs": {
                "file_types": [(".OGG", "*.ogg"), ("ALL Files", "*.*")]
            }
        }
    ]},
    {"text": "Transcript of your reaction: ", "input": [
        {
            "type": sg.InputText,
            "default_value": "",
            "param_name": "response_transcript"
        }
    ]},
    {"input": [
        {
            "type": sg.Checkbox,
            "default_value": "Includes Player Name",
            "kwargs": {"default": "existing_data", "regular_default": True},
            "param_name": "includesPlayerName"
        }
    ]}, {"input": [
        {
            "type": sg.Checkbox,
            "default_value": "Includes Adult Content",
            "kwargs": {"default": "existing_data", "regular_default": False},
            "param_name": "x"
        }
    ]}, {"input": [
        {
            "type": sg.Checkbox,
            "default_value": "Content is US-Specific",
            "kwargs": {"default": "existing_data", "regular_default": False},
            "param_name": "us"
        }
    ]}],
    "content_list": [
        {"type": "json"},
        {"type": "CustomData", "func": champd_up_data_jet, "kwargs": {}},
        {"type": "files", "files": {
            "args": [{"path": "param_name", "param_name": "contest_path", "name": "contest.ogg"}],
            "kwargs": {"adding_other_files": True}
        }}
    ]
}

champd_up_round_1 = CustomContentWindow("WorldChampions", "WorldChampionsRound", "contest", champd_up_round_layout)

champd_up_round_2_5_layout = {
    "previous_window": "champd_up",
    "layout_list": [{"text": "Contest Name: ", "input": [
        {
            "type": sg.InputText,
            "default_value": "The Champion of <ANYPLAYER>'s Nightmares",
            "param_name": "contest"
        }
    ]}, {"text": "A shorter description (that doesn't include the word \"Champion\"): ", "input": [
        {
            "type": sg.InputText,
            "default_value": "<ANYPLAYER>'s Nightmares",
            "param_name": "gameText"
        }
    ]}, {"text": ".OGG file of you reading the contest name (recommended)", "input": [
        {
            "type": sg.InputText,
            "default_value": "",
            "param_name": "contest_path"
        }, {
            "type": sg.FileBrowse,
            "default_value": "Browse",
            "param_name": "contest_browse",
            "kwargs": {
                "file_types": [(".OGG", "*.ogg"), ("ALL Files", "*.*")]
            }
        }
    ]}, {"input": [
        {
            "type": sg.Checkbox,
            "default_value": "Includes Player Name",
            "kwargs": {"default": "existing_data", "regular_default": True},
            "param_name": "includesPlayerName"
        }
    ]}, {"input": [
        {
            "type": sg.Checkbox,
            "default_value": "Includes Adult Content",
            "kwargs": {"default": "existing_data", "regular_default": False},
            "param_name": "x"
        }
    ]}, {"input": [
        {
            "type": sg.Checkbox,
            "default_value": "Content is US-Specific",
            "kwargs": {"default": "existing_data", "regular_default": False},
            "param_name": "us"
        }
    ]}],
    "content_list": [
        {"type": "json"},
        {"type": "CustomData", "func": champd_up_data_jet, "kwargs": {}},
        {"type": "files", "files": {
            "args": [{"path": "param_name", "param_name": "contest_path", "name": "contest.ogg"}],
            "kwargs": {"adding_other_files": True}
        }}
    ]
}

def champd_up_round_import(values):
    new_values = values
    new_values["linkedPrompts"] = "|".join(values["linkedPrompts"])
    return new_values

def champd_up_round_filter(values):
    new_values = values
    new_values["linkedPrompts"] = values["linkedPrompts"].split("|")
    return new_values

champd_up_round_2 = CustomContentWindow("WorldChampions", "WorldChampionsSecondHalfA", "contest", {
    "previous_window": "champd_up",
    "layout_list": [{"text": "Contest Name: ", "input": [
        {
            "type": sg.InputText,
            "default_value": "The Champion of <ANYPLAYER>",
            "param_name": "contest"
        }
    ]}, {"text": "A shorter description (that doesn't include the word \"Champion\"): ", "input": [
        {
            "type": sg.InputText,
            "default_value": "<ANYPLAYER>'s Champion",
            "param_name": "gameText"
        }
    ]}, {"text": "List of connected prompts (separate by |): ", "input": [
        {
            "type": sg.InputText,
            "default_value": "500000|500002|500120",
            "param_name": "linkedPrompts"
        }
    ]}, {"text": ".OGG file of you reading the contest name (recommended)", "input": [
        {
            "type": sg.InputText,
            "default_value": "",
            "param_name": "contest_path"
        }, {
            "type": sg.FileBrowse,
            "default_value": "Browse",
            "param_name": "contest_browse",
            "kwargs": {
                "file_types": [(".OGG", "*.ogg"), ("ALL Files", "*.*")]
            }
        }
    ]}, {"text": ".OGG file of you reacting to the prompt (something like 'Oooh! Scary!') (recommended)", "input": [
        {
            "type": sg.InputText,
            "default_value": "",
            "param_name": "response_path"
        }, {
            "type": sg.FileBrowse,
            "default_value": "Browse",
            "param_name": "response_browse",
            "kwargs": {
                "file_types": [(".OGG", "*.ogg"), ("ALL Files", "*.*")]
            }
        }
    ]}, {"text": "Transcript of your reaction: ", "input": [
        {
            "type": sg.InputText,
            "default_value": "",
            "param_name": "response_transcript"
        }
    ]}, {"input": [
        {
            "type": sg.Checkbox,
            "default_value": "Includes Player Name",
            "kwargs": {"default": "existing_data", "regular_default": True},
            "param_name": "includesPlayerName"
        }
    ]}, {"input": [
        {
            "type": sg.Checkbox,
            "default_value": "Includes Adult Content",
            "kwargs": {"default": "existing_data", "regular_default": False},
            "param_name": "x"
        }
    ]}, {"input": [
        {
            "type": sg.Checkbox,
            "default_value": "Content is US-Specific",
            "kwargs": {"default": "existing_data", "regular_default": False},
            "param_name": "us"
        }
    ]}],
    "content_list": [
        {"type": "json"},
        {"type": "CustomData", "func": champd_up_data_jet, "kwargs": {}},
        {"type": "files", "files": {
            "args": [{"path": "param_name", "param_name": "contest_path", "name": "contest.ogg"}],
            "kwargs": {"adding_other_files": True}
        }}
    ],
    "filter": champd_up_round_filter,
    "import_filter": champd_up_round_import
})

champd_up_round_2_5 = CustomContentWindow("WorldChampions", "WorldChampionsSecondHalfB", "contest", champd_up_round_2_5_layout)

champd_up = SelectionWindow("Champ'd Up Content Selection", ["Please select the type of content", ("Round 1 Prompt", "Round 2 Prompt", "Round 2.5 Prompt"), "champd_up_content_type"], {
    "Round 1 Prompt": champd_up_round_1.create_window,
    "Round 2 Prompt": champd_up_round_2.create_window,
    "Round 2.5 Prompt": champd_up_round_2_5.create_window
}, "create_content")

#Blather Round stuff

def blather_round_word_filter(values):
    new_values = values
    new_values["alternateSpellings"] = values["alternateSpellings"].split("|")
    new_values["forbiddenWords"] = values["forbiddenWords"].split("|")
    if new_values["forbiddenWords"] == [""]:
        new_values["forbiddenWords"] = []
    tailored_words = []
    old_tailored_words = values["tailoredWords"].split("|")
    for i in range(0, len(old_tailored_words), 2):
        tailored_words.append({"list": old_tailored_words[i], "word": old_tailored_words[i + 1]})
    new_values["tailoredWords"] = tailored_words
    return new_values

def blather_round_word_import(values):
    new_values = values
    new_values["alternateSpellings"] = "|".join(values["alternateSpellings"])
    new_values["forbiddenWords"] = "|".join(values["forbiddenWords"])
    tailored_words_str = ""
    for index, item in enumerate(values["tailoredWords"]):
        tailored_words_str += item["list"] + "|" + item["word"] + ("|" if index < len(values["tailoredWords"]) - 1 else "")
    new_values["tailoredWords"] = tailored_words_str
    return new_values

def blather_round_word_data_jet(values):
    data = CustomData()
    data.add_data("S", values["password"], "Password")
    data.add_data("S", values["category"], "Category")
    if values["subcategory"] != "":
        data.add_data("S", values["subcategory"], "Subcategory")
    data.add_data("S", values["difficulty"], "Difficulty")
    data.add_data("S", "|".join(values["forbiddenWords"]), "ForbiddenWords")
    data.add_data("S", "|".join(values["alternateSpellings"]), "AlternateSpellings")
    return data

blather_round_word = CustomContentWindow("BlankyBlank", "BlankyBlankPasswords", "password", {
    "previous_window": "blather_round",
    "layout_list": [{"text": "Word/Phrase to use: ", "input": [
        {
            "type": sg.InputText,
            "default_value": "R'lyeh",
            "param_name": "password"
        }
    ]}, {"text": "Alternate spellings (separate by |): ", "input": [
        {
            "type": sg.Multiline,
            "default_value": "Rlyeh|Ryleh|R'yleh|R'leyh|Rehly|Rl'yeh|Ry'leh|Relyh|Rly'eh",
            "param_name": "alternateSpellings"
        }
    ]}, {"text": "Word/Phrase Category: ", "input": [
        {
            "type": sg.InputText,
            "default_value": "place",
            "param_name": "category"
        }
    ]}, {"text": "Subcategory (optional): ", "input":[
        {
            "type": sg.InputText,
            "default_value": "city",
            "param_name": "subcategory"
        }
    ]}, {"text": "Difficulty (easy, medium, or hard): ", "input": [
        {
            "type": sg.InputText,
            "default_value": "hard",
            "param_name": "difficulty"
        }
    ]}, {"text": "Forbidden Words (separate by |): ", "input": [
        {
            "type": sg.Multiline,
            "default_value": "",
            "param_name": "forbiddenWords"
        }
    ]}, {"text": "Tailored Words (separate by |): ", "input": [
        {
            "type": sg.Multiline,
            "default_value": "<nature-places>|underwater|<nature-places>|sea|<abstract-concept>|insanity|<abstract-concept>|evil|<genre>|horror|<genre>|fiction",
            "param_name": "tailoredWords"
        }
    ]}, {"input": [
        {
            "type": sg.Checkbox,
            "default_value": "Content is US-Specific",
            "kwargs": {"default": "existing_data", "regular_default": False},
            "param_name": "us"
        }
    ]}],
    "content_list": [
        {"type": "json"},
        {"type": "CustomData", "func": blather_round_word_data_jet, "kwargs": {}}
    ],
    "filter": blather_round_word_filter,
    "import_filter": blather_round_word_import
})

def blather_round_category_filter(values):
    new_values = values
    new_values["structures"] = values["structures"].split("|")
    return new_values

def blather_round_category_import(values):
    new_values = values
    new_values["structures"] = "|".join(values["structures"])
    return new_values

def blather_round_category_data_jet(values):
    data = CustomData()
    data.add_data("S", values["category"], "Category")
    data.add_data("S", "|".join(values["structures"]), "Structures")
    return data

blather_round_category = CustomContentWindow("BlankyBlank", "BlankyBlankSentenceStructures", "category", {
    "previous_window": "blather_round",
    "layout_list": [{"text": "Broad Category Name: ", "input": [
        {
            "type": sg.InputText,
            "default_value": "art",
            "param_name": "category"
        }
    ]}, {"text": "Sentence Structures (separate by |): ", "input": [
        {
            "type": sg.Multiline,
            "default_value": "It's <article-1> <art-adjective-simple> <art-noun-simple>.|It <art-verb-simple> <article-2> <art-noun-simple>.|Talk about <art-adjective-complex>!|It <art-verb-simple> <article-2> <art-noun-complex>.|Oh, <art-noun-complex>!|Quite simply, it's <article-1> <art-adjective-complex> <art-noun-complex>.",
            "param_name": "structures",
            "kwargs": {"size": (50, 10)}
        }
    ]}],
    "content_list": [
        {"type": "json"},
        {"type": "CustomData", "func": blather_round_category_data_jet, "kwargs": {}}
    ],
    "filter": blather_round_category_filter,
    "import_filter": blather_round_category_import
})

def blather_round_descriptor_filter(values):
    new_values = values
    new_values["amount"] = ""
    new_values["optional"] = False
    words_list = values["words"].split("|")
    new_words = []
    next_true = False
    for item in words_list:
        if item == "T":
            next_true = True
        else:
            new_words.append({"word": item, "alwaysChoose": next_true})
            if next_true == True:
                next_true = False
    new_values["words"] = new_words
    return new_values

def blather_round_descriptor_import(values):
    new_values = values
    words_str = ""
    for index, item in enumerate(new_values["words"]):
        if item["alwaysChoose"] == True:
            words_str += ("|" if index != 0 else "") + "T|"
        words_str += item["word"] + ("|" if index < len(new_values["words"]) - 1 else "")
    new_values["words"] = words_str
    return new_values

def blather_round_descriptor_data_jet(values):
    data = CustomData()
    data.add_data("S", values["name"], "Name")
    data.add_data("B", values["optional"], "Optional")
    data.add_data("S", values["amount"], "Amount")
    data.add_data("S", values["maxChoices"], "MaxChoices")
    data.add_data("S", values["placeholder"], "Placeholder")
    return data

blather_round_descriptor = CustomContentWindow("BlankyBlank", "BlankyBlankWordLists", "name", {
    "previous_window": "blather_round",
    "layout_list": [{"text": "Descriptor name (hyphenate, please): ", "input": [
        {
            "type": sg.InputText,
            "default_value": "hyphenated-descriptor (Please read README)",
            "param_name": "name"
        }
    ]}, {"text": "Words List: ", "input": [
        {
            "type": sg.Multiline,
            "default_value": "list|of|separated|T|words|or|sentences",
            "param_name": "words",
            "kwargs": {"size": (50, 5)}
        }
    ]}, {"text": "Maximum number of choices (leave blank for none): ", "input": [
        {
            "type": sg.InputText,
            "default_value": "",
            "param_name": "maxChoices"
        }
    ]}, {"text": "Placeholder text (blank, blanks, or blanky): ", "input": [
        {
            "type": sg.InputText,
            "default_value": "blank",
            "param_name": "placeholder"
        }
    ]}],
    "content_list": [
        {"type": "json"},
        {"type": "CustomData", "func": blather_round_descriptor_data_jet, "kwargs": {}}
    ],
    "filter": blather_round_descriptor_filter,
    "import_filter": blather_round_descriptor_import
})

blather_round = SelectionWindow("Blather 'Round Content Selection", ["Please select the type of content (Read Readme)", ("Word", "Category", "Descriptors"), "blather_round_content_type"], {
    "Word": blather_round_word.create_window,
    "Category": blather_round_category.create_window,
    "Descriptors": blather_round_descriptor.create_window
}, "create_content")

#Main Menu stuff

create_content = SelectionWindow("Select a game", ["Select a game.", ("Blather 'Round", "Champ'd Up", "Quiplash 3", "Talking Points"), "game"],{
    "Blather 'Round": blather_round.run,
    "Talking Points": talking_points.run,
    "Quiplash 3": quiplash_3.run,
    "Champ'd Up": champd_up.run
}, "main_window")

main_window = SelectionWindow("Select an option", ["Please select an option.", ("Create Custom Content", "View/Edit Content", "Import/Reimport Content", "Only Use Custom Content"), "option"], {
    "Create Custom Content": create_content.run,
    "View/Edit Content": edit_content,
    "Import/Reimport Content": import_content_window,
    "Only Use Custom Content": del_all_else
})

window_mapping = { #Used for backing out of stuff.
    "quiplash_prompt": quiplash_prompt,
    "quiplash_3": quiplash_3,
    "create_content": create_content,
    "main_window": main_window,
    "talking_points": talking_points,
    "champd_up": champd_up,
    "blather_round": blather_round,
    "edit_content": edit_content
}

content_type_mapping = { #Used in editing content to change data.
    "BlankyBlank": {
        "BlankyBlankPasswords": blather_round_word,
        "BlankyBlankSentenceStructures": blather_round_category,
        "BlankyBlankWordLists": blather_round_descriptor 
    },
    "JackboxTalks":{
        "JackboxTalksPicture": talking_points_picture,
        "JackboxTalksTitle": talking_points_prompt,
        "JackboxTalksSignpost": talking_points_slide_transition
    },
    "Quiplash3":{
        "Quiplash3Round1Question": round_prompt_1,
        "Quiplash3Round2Question": round_prompt_2,
        "Quiplash3FinalQuestion": round_prompt_final,
        "Quiplash3SafetyQuips": safety_quip
    },
    "WorldChampions": {
        "WorldChampionsRound": champd_up_round_1,
        "WorldChampionsSecondHalfA": champd_up_round_2,
        "WorldChampionsSecondHalfB": champd_up_round_2_5
    }
}

main_window.run()