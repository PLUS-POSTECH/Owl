import React from "react";
import { Header } from "semantic-ui-react";
import { Line } from "react-chartjs-2";

import { game_state } from "./game-state";

export const ScoreboardPath = "/scoreboard";

const RoundDisplay: React.FC = () => {
  return <Header as="h1" textAlign="center">
    Current Tick: {game_state.current_tick}
  </Header>
};

const ScoreTimeline: React.FC = () => {
  let randomColorGenerator = function() {
    const red = Math.floor(Math.random() * 256);
    const green = Math.floor(Math.random() * 256);
    const blue = Math.floor(Math.random() * 256);
    return [
      `rgba(${red},${green},${blue},0.7)`,
      `rgba(${red},${green},${blue},0.3)`
    ];
  };

  const teams_dict: {[id: string]: any} = {};

  for (const team of game_state.teams) {
    let color = randomColorGenerator();
    teams_dict[team.id] = {
      id: team.id,
      label: team.name,
      fill: false,
      tension: 0,
      borderColor: color[0],
      backgroundColor: color[1],
      data: []
    }
  }

  let labels = [];
  for (let tick = 0; tick < game_state.scores.length; tick++) {
    labels.push(String(tick));
    const score_data = game_state.scores[tick];
    const ranks: {[rank: string]: {"ATTACK": number, "DEFENSE": number, "KING_OF_THE_HILL": number, "id": number}} = score_data.teams
    for (const rank in ranks) {
      let team_score = ranks[rank];
      teams_dict[team_score.id].data.push(team_score.ATTACK + team_score.DEFENSE + team_score.KING_OF_THE_HILL);
    }
  }

  let teams_arr = [];
  for (const id in teams_dict) {
    teams_arr.push(teams_dict[id]);
  }

  return (
    <Line
      data={{
        labels: labels,
        datasets: teams_arr
      }}
      options={{
        scales: {
          xAxes: [
            {
              display: true,
              scaleLabel: {
                display: true,
                labelString: "Tick"
              }
            }
          ],
          yAxes: [
            {
              display: true,
              scaleLabel: {
                display: true,
                labelString: "Score"
              }
            }
          ]
        }
      }}
    />
  );
};

export const Scoreboard: React.FC = () => (
  <div
    style={{
      marginLeft: 40,
      marginRight: 40
    }}
  >
    <RoundDisplay />
    <ScoreTimeline />
  </div>
);
