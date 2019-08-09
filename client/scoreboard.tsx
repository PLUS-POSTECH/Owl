import React, { useState } from "react";
import { Header, Form, Checkbox, Container } from "semantic-ui-react";
import { Line } from "react-chartjs-2";

import { game_state } from "./game-state";

export const ScoreboardPath = "/scoreboard";

const RoundDisplay: React.FC = () => {
  return <Header as="h1" textAlign="center">
    Scoreboard for tick {game_state.current_tick}
  </Header>
};

const ScoreTimeline: React.FC = () => {
  const [perRound, setPerRound] = useState(false);
  const [addAttack, setAddAttack] = useState(true);
  const [addDefense, setAddDefense] = useState(true);
  const [addKoth, setAddKoth] = useState(true);

  function HSVtoRGB(h: number, s: number, v: number) {
    let r = 0, g = 0, b = 0, i, f, p, q, t;
    i = Math.floor(h * 6);
    f = h * 6 - i;
    p = v * (1 - s);
    q = v * (1 - f * s);
    t = v * (1 - (1 - f) * s);
    switch (i % 6) {
      case 0: r = v, g = t, b = p; break;
      case 1: r = q, g = v, b = p; break;
      case 2: r = p, g = v, b = t; break;
      case 3: r = p, g = q, b = v; break;
      case 4: r = t, g = p, b = v; break;
      case 5: r = v, g = p, b = q; break;
    }
    return {
      r: Math.round(r * 255),
      g: Math.round(g * 255),
      b: Math.round(b * 255)
    };
  }

  let colorGenerator = function (team_idx: number) {
    const result = HSVtoRGB(team_idx / game_state.teams.length, 1, 0.7);
    return [
      `rgba(${result.r},${result.g},${result.b},0.7)`,
      `rgba(${result.r},${result.g},${result.b},0.3)`
    ];
  };

  const teams_dict: { [id: string]: any } = {};

  for (const team of game_state.teams) {
    let color = colorGenerator(Number(team.id));
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
    const ranks: { [rank: string]: { "ATTACK": number, "DEFENSE": number, "KING_OF_THE_HILL": number, "id": number } } = score_data.teams
    for (const rank in ranks) {
      let team_score = ranks[rank];

      let current_score = 0;
      if (addAttack) {
        current_score += team_score.ATTACK;
      }
      if (addDefense) {
        current_score += team_score.DEFENSE;
      }
      if (addKoth) {
        current_score += team_score.KING_OF_THE_HILL;
      }

      if (perRound || teams_dict[team_score.id].data.length == 0) {
        teams_dict[team_score.id].data.push(current_score);
      } else {
        let target_data = teams_dict[team_score.id].data;
        let last_score = target_data[target_data.length - 1];
        teams_dict[team_score.id].data.push(last_score + current_score);
      }
    }
  }

  let teams_arr = [];
  for (const id in teams_dict) {
    teams_arr.push(teams_dict[id]);
  }

  return (
    <>
      <Container text textAlign="center">
        <Form>
          <Form.Field>
            <Checkbox label="Per Round" checked={perRound} onClick={() => setPerRound(!perRound)} />
          </Form.Field>
          <Form.Field>
            <Checkbox label="Attack" checked={addAttack} onClick={() => setAddAttack(!addAttack)} />
          </Form.Field>
          <Form.Field>
            <Checkbox label="Defense" checked={addDefense} onClick={() => setAddDefense(!addDefense)} />
          </Form.Field>
          <Form.Field>
            <Checkbox label="King of the Hill" checked={addKoth} onClick={() => setAddKoth(!addKoth)} />
          </Form.Field>
        </Form>
      </Container>
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
    </>
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
