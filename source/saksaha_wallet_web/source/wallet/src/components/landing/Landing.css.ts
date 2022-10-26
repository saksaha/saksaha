import { style } from '@vanilla-extract/css';

export const wrapper = style({
  width: "100%",
  height: "100%",
  display: "flex",
  border: "1px solid #010101",
});

export const left_pane = style({
  width: "30%",
  height: "100%",
  float: "right",
  boxSizing: "border-box",
  background: "#ece6cc",
  border: "3px solid yellow",
  paddingTop: "240px",
  paddingLeft: "50px",
  paddingRight: "50px",
})

export const right_pane = style({
  width: "70%",
  height: "100%",
  float: "left",
  boxSizing: "border-box",
  background: "#8977ad",
  border: "3px solid green",
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
  textAlign: "center",
  paddingTop: "50px",
  paddingLeft: "50px",
  paddingRight: "50px",
});

