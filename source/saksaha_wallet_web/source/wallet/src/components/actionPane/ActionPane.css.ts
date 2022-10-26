import { style } from '@vanilla-extract/css';

export const InputWalletString = "  [1] Input your ID (secret key, nickname, etc...)";
export const InputData = "  [2] Select Coin, MRS Slots, and input your Data";

export const ex1 = style({
  color: 'blue',
});

export const input_row = style({
  display: "flex",
  flexDirection: "row",

  alignItems: "center",
  textAlign: "center",
  justifyContent: "space-between",

  marginTop: "10px",
})

export const input_single_field = style({
  width: "80%",
  height: "50px",

  border: "1px solid black",
  borderRadius: "1px",

  fontSize: "17px",

  ":placeholder-shown":
  {
    fontWeight: "bold",
    opacity: ".5",
    color: "red",
  }
})

export const input_two_field = style({
  width: "35%",
  height: "50px",

  border: "1px solid black",
  borderRadius: "1px",

  fontSize: "17px",


  ":placeholder-shown":
  {
    fontWeight: "bold",
    opacity: ".5",
    color: "red",
  }
})

export const input_btn = style({
  width: "50px",
  height: "48px",

  backgroundColor: "#f5d682",

  border: "2px solid grey",

  borderRadius: "1px",

  marginLeft: "80px",

  fontSize: "14px",

  lineHeight: "48px",


  ":hover": {
    color: "black",
    backgroundColor: "#d8af28",
    border: "2px solid red",
    cursor: "pointer",
    // boxShadow: "2px 2px black",
  }
})
