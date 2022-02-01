from os import walk


(_, _, files) = walk("src/resources/generated/").next()

structs_by_name = {}

def parse_struct(struct, file):
    # does nothing clever involving lifetime parameters, type parameters, etc. Considers them part of the name
    # I was expecting to have to do more normalizing work, but luckily since the code is generated even this naive solution does the job
    
    clean_struct = ""
    # first split by newlines to remove macro attributes and comments
    for line in struct.split("\n"):
        line = line.strip()
        if line.find("//") != -1:
            line = line[0:line.find("//")]
        if line.find("#") != -1:
            line = line[0:line.find("#")]
        if line != "":
            clean_struct += line

    header = clean_struct[0: clean_struct.find("{") + 1].split(" ")
    clean_struct = clean_struct[clean_struct.find("{") + 1: clean_struct.rfind("}")]
    struct_identifier = clean_struct
    name = ""

    for word in header:
        if word != 'struct' and word!='{' and word != 'pub':
            name = word
            break


    if structs_by_name.get(name) != None:
        print("found "+name+" "+str(len(structs_by_name[name]["present_in"])+1)+" times, first found in "+structs_by_name[name]["present_in"][0]+" now found in "+file)
        structs_by_name[name]["present_in"] += file
        return (True, name)

    structs_by_name[name] = {"present_in": [file]}
    return (False, None)

for filename in files: 
    #print("on "+filename)
    with open("src/resources/generated/"+filename) as fi:
        struct_start = 0;
        struct_end = 0;
        content = fi.read()

        removals = []
        while struct_start != -1:

            # look for a struct
            struct_start = content.find("struct ", struct_end)

            # look for open braces
            inside_open_braces = content.find("{", content.find("\n",struct_start));

            # look for a matching closing brace
            struct_end = content.find("}", struct_start ) +1

            while inside_open_braces < struct_end and inside_open_braces != -1:
                struct_end = content.find("}", struct_end ) +1
                inside_open_braces = content.find("{", inside_open_braces +1);

            # parse and store the stuct
            if struct_start != -1:
                (remove, name) = parse_struct(content[ struct_start: struct_end ], filename)
                if remove:
                    removals.append({"name": name, "struct_start": struct_start, "struct_end": struct_end})

        imports = ""
        removed_before = 0
        for removal in removals:
            content = content[:removal["struct_start"]-removed_before] + content[removal["struct_end"]-removed_before:]
            imports += removal.name+", "

        content = "use crate::resources::{"+imports+"}"+content;
        print(content)

