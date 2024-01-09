# Anki Lang Gen

Language learning requires immersion. However, it's often difficult to find material at your reading level or to reinforce the exact words that you're learning at any given time. 

I built this tool to solve that problem. It reads from your anki deck and retrieves any cards that are reviewed. It then extracts the word from the question and creates a list of them passing it ultimately into gpt4 along with your target language to generate a short story at your reading level.

**Note**: Because each anki deck is unique, the parsing logic that I wrote is custom to my specific deck that I'm studying. If you want help writing the parsing logic for the deck you're working on, I would be happy to help. Just raise an issue or dm me on twitter. 




## Environment Variables

To run this project, you will need to export the environment variable with your gpt4 api key.

`export OPENAI_API_KEY='sk-...'`


## Future ideas

- Write a proc macro that writes the parsing logic for you based on the actual contents of the deck by passing the deck contents into gpt4 and having it write the rust code :D


## Requirements

You need to have the anki connect plugin installed: https://ankiweb.net/shared/info/2055492159

With this installed, anki will expose a rest api.
