import { style } from "@vanilla-extract/css";

export const coin_record = style({
  border: "3px solid green",
  borderRadius: "5px",
  width: "fit-content",
  height: "fit-content",
  textAlign: "left",
  fontSize: "14px",
  fontWeight: "bold",
  marginTop: "10px",
  padding: "5px",

  ":hover": {
    border: "3px solid red",
    background: "gainsboro",
  },
});

export const selected_coin_record = style({
  border: "3px solid red",
  background: "gainsboro",
  borderRadius: "5px",
  width: "fit-content",
  height: "fit-content",
  textAlign: "left",
  fontSize: "14px",
  fontWeight: "bold",
  marginTop: "10px",
  padding: "5px",
});

export const result = style({
  width: "100%",
  height: "100%",
  background: "skyblue",
  border: "3px solid orange",
  display: "flex",
  flexDirection: "row",
  alignItems: "center",
});

export const result_coin_pane = style({
  width: "50%",
  height: "100%",
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
});

export const result_mrs_slot_pane = style({
  width: "50%",
  height: "100%",
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
});

export const mrs_slot = style({
  border: "3px solid green",
  borderRadius: "5px",
  width: "180px",
  height: "30px",
  textAlign: "center",
  lineHeight: "30px",
  fontSize: "14px",
  fontWeight: "bold",
  marginTop: "10px",

  ":hover": {
    border: "3px solid red",
    background: "gainsboro",
  },
});

export const selected_mrs_slot = style({
  border: "3px solid red",
  background: "gainsboro",
  borderRadius: "5px",
  width: "fit-content",
  height: "fit-content",
  textAlign: "left",
  fontSize: "14px",
  fontWeight: "bold",
  marginTop: "10px",
});

export const vertical_line = style({
  width: "2px",
  height: "100%",
  background: "orange",
});
