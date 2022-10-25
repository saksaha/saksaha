import { style } from '@vanilla-extract/css';

export const InputWalletString = "  [1] Input your ID (secret key, nickname, etc...)";
export const SlotIdString = "  [2] Slot Id";
export const SlotDataString = "  [2] Data";

export const ex1 = style({
  color: 'blue',
});


export const wrapper = style({
  width: "100%",
  height: "100%",
  display: "flex",
  border: "1px solid #010101",
});



export const result = style({
  width: "100%",
  height: "100%",

  background: "skyblue",
  border: "3px solid orange",

  marginBottom: "30px",

});

export const result_body = style({
  height: "100%",
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
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

export const coin_record = style({
  border: "1px solid green",

  borderRadius: "5px",

  width: "fit-content",
  height: "fit-content",

  textAlign: "left",

  fontSize: "14px",

  fontWeight: "bold",

  marginTop: "10px",

  ":hover": {
    border: "1px solid red",

    background: "gainsboro"
  }
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
