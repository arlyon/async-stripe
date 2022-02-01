from os import walk

# step 1: find structs, enums that conflict
# step 2: for every struct and enum that conflicts, record impl's
# step 3: remove all duplicate stucts and enums except for the first. Use from crate::resources in files where it was removed. Place all unique impl's inside the file where we now have the struct

structs_unique = {}
enums_unique = {}

def get_struct_header(struct):
    header = []
    if struct.find("struct ") == -1:
        header = struct[struct.find("enum ")+5:struct.find("{")]
    else:
        header = struct[struct.find("struct ")+7:struct.find("{")]
    return header

def get_struct_name(struct):
    header = get_struct_header(struct).split()
    print(header)
    return strip_angles(header[0], 0)

def strip_angles(str, i):
    first = str.find('<', i)
    if first == -1:
        return str

    left = 0
    right = 0
    offset = -1
    for i in range(first, len(str)):
        if str[i] == '<':
            left += 1
        if str[i] == '>':
            right+= 1
        if left == right:
            offset = i+1
            break
    return str[:first] + str[first+offset:]


def is_related(impl, name):
    header = impl[impl.find("impl")+4:impl.find("{")]

    header = strip_angles(header, 0)

    header = header.strip().split()
    impl_struct = header[0]

    for i in range(len(header)):
        if header[i] == 'for':
            impl_struct = header[i+1]

    if strip_angles(impl_struct, 0) == name: 
        print("impl "+header[0]+" is related to "+name)

    return strip_angles(impl_struct, 0) == name

# onelines and otherwise removes comments etc on text
def trim(text):
    clean_text = ""

    for line in text.strip().split("\n"):
        line = line.strip()
        if line.find("//") != -1:
            line = line[0:line.find("//")]
        if line.find("#") != -1:
            line = line[0:line.find("#")]
        if line != "":
            clean_text += line
    return clean_text

def removal_sort(item):
    return item["block_start"] # there should never be overlap

def find_block_end(content, block_start):
    # look for open braces
    inside_open_braces = content.find("{", content.find("\n",block_start))

    # look for a matching closing brace
    block_end = content.find("}", block_start ) +1

    while inside_open_braces < block_end and inside_open_braces != -1:
        block_end = content.find("}", block_end ) +1
        inside_open_braces = content.find("{", inside_open_braces +1)

    return block_end

# we may be better of parsing, lexing, and doing a more thorough job, but this works well enough. The code generated by openapi can be very predictable, which allows some of the assumptions made below to hold true
def parse_content(content):
    # find every struct, enum, and impl
    block_start = 0
    block_end = 0

    removals = []
    structs = []
    impls = []
    while block_start != -1:
        #print(block_end)

        #TODO: include derive macros on top
        # look for a struct
        block_start = content.find("pub struct ", block_end)
        kind = "struct"
        
        # impl doesn't have a trailing space because it can have generics
        if content.find("impl", block_end) != -1 and content.find("impl", block_end) < block_start:
            block_start = content.find("impl", block_end)
            kind = "impl"

        if content.find("pub enum ", block_end) != -1 and content.find("pub enum ", block_end) < block_start:
            block_start = content.find("pub enum ", block_end)
            kind = "enum"

        
        block_end = find_block_end(content, block_start)
        # adjust the upper bound of the effective block by searching backwards for macros
        # TODO: loop through this with bounds
        #print("macros")
        macro_start = block_start
        while True:
            prev_line = [content.rfind("\n", 0, macro_start-1), macro_start-1]
            if prev_line[0] == -1 or prev_line[1] == -1:
                break
            prev_macro = content.find("#", prev_line[0], prev_line[1])
            if prev_macro == -1:
                prev_macro = content.find("///", prev_line[0], prev_line[1])
            if prev_macro == -1:
                break

            print("macro "+str(block_start)+" "+str(prev_line[0])+" "+str(prev_line[1]))
            print("prev macro line "+content[prev_line[0]: prev_line[1]])
            if prev_macro!=-1:
                macro_start=prev_macro

        if content[block_start: block_end].strip() == "":
            continue

        if kind == "struct" or kind == "enum":
            name = get_struct_name(content[block_start: block_end])
            if name.strip() == "":
                continue
            
            structs.append({"macro_start": macro_start, "block_start":block_start,"block_end":block_end, "kind": kind, "file": filename})
            print("found data" + content[macro_start: block_end])
        if kind == "impl":
            impls.append({"macro_start": macro_start, "block_start":block_start,"block_end":block_end,  "kind": kind, "file": filename})

    # insert structs and enums into dict. If the insertion tells us about a conflict, add the struct/enum to imports and be prepared to remove all impls that pertain to this struct/enum
    for struct in structs:
        with_macros = content[struct["macro_start"]: struct["block_end"]]
        struct_text = content[struct["block_start"]: struct["block_end"]]
        identifier = hash(trim(with_macros))

        if struct_text.strip() == "":
            continue

        # get the name so we know which impls to look for
        name = get_struct_name(struct_text)

        if name == "": #something is wrong
            continue

        if structs_unique.get(identifier) == None:
            structs_unique[identifier] = { "duplicates": [], "related_impls": {}, "text": with_macros, "kind": struct["kind"], "name": get_struct_name(struct_text), "header": get_struct_header(struct_text) }
            print("found first "+name)
        else:
            removals.append(struct)
            print("found duplicate "+name)

        structs_unique[identifier]["duplicates"].append(struct)

        # find all relevant impls in file

        # problem: doesn't apply to impl's we haven't found yet
        # for impl's, find out if they pertain to any of the structs or enums we just inserted. If they do, insert them with their associated struct/enum. 
        for impl in impls:
            impl_text_with_macros = content[impl["macro_start"]: impl["block_end"]]
            impl_text = content[impl["block_start"]: impl["block_end"]]
            if is_related(impl_text, name):
                # we are definitely removing this impl! That's why we don't have to keep track of duplicates, it doesn't matter if we're the first or the second or the last
                structs_unique[identifier]["related_impls"][hash(trim(impl_text_with_macros))] = {"location":impl, "file": filename, "text":impl_text_with_macros}

                # remove the impl from the current file, regardless
                removals.append(impl)

    # for each file, remove the removals and add the imports
    removed_offset = 0
    removals.sort(reverse=True, key=removal_sort)
    imports = ""
    for removal in removals:
        if removal["kind"] == "struct" or removal["kind"] == "enum":
            # TODO: add imports
            name = get_struct_name(content[removal["block_start"]:removal["block_end"]])
            imports += name+", "
            print("removing "+removal["kind"]+" "+name)
        else:
            print("removing "+content[removal["block_start"]:content.find("\n", removal["block_start"])])
        content = content[:removal["macro_start"]] + content[removal["block_end"]:]

    f = open("src/resources/generated/"+filename, "w")
    if imports!= "":
        f.write("use crate::resources::{"+imports+"};\n"+content)
    else: 
        f.write(content)

    f.close()


def add_back_impls():
    # finally, do one last pass through the dictionary to add impls back to their files. Make some effort to insert them near the stucts
    
    # for every struct in structs_unique
    for struct in structs_unique:
        # get the first duplicate
        print("adding back "+str(struct))
        filename = structs_unique[struct]["duplicates"][0]["file"]
        f = open("src/resources/generated/"+filename, "r")
        print("reading file");
        content = f.read()
        oldlen = len(content)
        f.close()

        f = open("src/resources/generated/"+filename, "w")
        # for now we'll just stick the impl's at the bottom
        # block_end indices are invalid beacuse of the removals process

        impls_stringified = ""

        print("finding block end");
        end = find_block_end(content, content.find(structs_unique[struct]["header"]))
        if end == -1:
            print("failed")
            exit(1)

        print("collecting related impls");
        for impl in structs_unique[struct]["related_impls"]:
            struct_name = structs_unique[struct]["name"]
            impls_stringified += "\n//automatically added back in service of "+struct_name+" with hash"+str(impl)+"\n"+structs_unique[struct]["related_impls"][impl]["text"]+"\n"

        print("impls"+impls_stringified);
        content = content[:end]+impls_stringified+content[end:]
        print("end: "+str(end)+" writing "+str(len(content)-oldlen)+" new bytes to "+filename);
        f.write(content)
        f.close()
        del content
        del impls_stringified

    # find block end 
    # insert impls there
    # write the file
    # repeat

    

(_, _, files) = walk("src/resources/generated/").next()
for filename in files: 
    #print("on "+filename)
    with open("src/resources/generated/"+filename) as fi:
        block_start = 0
        block_end = 0
        content = fi.read()
        parse_content(content)

add_back_impls()
