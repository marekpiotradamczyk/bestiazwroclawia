import pandas as pd

#white_weights = pd.read_pickle('w2move_weights.pkl')
black_weights = pd.read_pickle('b2move_weights.pkl')

# Save the weights to JSON files
#white_weights.to_json('w2move_weights.json')
black_weights.to_json('b2move_weights.json')
