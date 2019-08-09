import requests

from datetime import datetime
import time


while True:
    print "Updating game state - {}".format(datetime.now().isoformat(' '))
    r = requests.get('http://10.13.37.8/game_state/game_state.json')

    with open('../client/game-state.ts', 'w') as f:
        f.write('export const game_state = {}'.format(r.text))

    time.sleep(30)
