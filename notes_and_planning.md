# File Reading
## Long form Rant
reading from files is easy, I think the hard part is making it efficient, i can't just call the "find setting" function for every setting and iterate over the file because that would take a fuck ton of resources, so instead I ought to have a defaults file, that i can compare the settings file to. and then go down the tree accordingly, so if i see something in the defaults that isn't in the custom config file, then I can just use the default version and then, after I know what im missing, i can create a final.config file that will be the default file but with the changed settings from the custom configuration file. The other option for this is only having the defaults file and having the user just edit that, but that seems both lazy and bad becuse if the user fucks up there's nothign to fall back to. 
## Shot 2do
- Create a defaults file (hidden)
- Create a final file (hidden)
- have file comparison (async?)
- somehow use that data as the variable data when rendering / setting keybinds in GPUI


# GPUI
has literally zero documentation, very sad, will end up using Azul instead because it's still rust native.