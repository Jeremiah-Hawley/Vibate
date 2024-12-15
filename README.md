# Vibate
docx editor with vim keybinds custumized for debate written in Rust
# TODO
## Setup Crates
1) Setup Tauri or Druid
2) choose between docx-rs and docx_rust for docx editing

# features
## docx editing with vim keybinds and "verbatim keybinds"
1) normal vim movement and editing keybinds
2) gf to go to .card file
3) add the ability to link cards to warrants in pre-written analytics and gd to jump to the card the warrant comes from
## Detect Tags, and create docx files with .card extention for cards
1) add .speech file which both stores a list of "decks" (one per adv), should display each card and be able to be exported as a .docx file
2) add "decks" of cards, which are .deck files with paths to each "card" in "deck"
## tag cards with attributes i.e. Link, Impact, Uniqueness, K...
1) should be on top of .card files
## have a search function, for specific cards, solves the "backfiles problem"
## have dropbox support
many options for implimentation, couldhave users input a path to the directory, or maybe there's a crate for that? I think i'll go with the path
## be able to parse existing docx files and split them into .card files

