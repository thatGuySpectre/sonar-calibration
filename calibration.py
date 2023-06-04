import serial
import PySimpleGUI as sg

font = ("Arial", 40)

layout = [
    [sg.T("Adjust Potetiometers until all sensors show red, and their values are the same (~520)", key="hint",
          font=("Arial", 25))],
    [sg.T(key="sensor0", font=font, size=(15, 5), background_color="red", justification="c"),
     sg.T(key="sensor1", font=font, size=(15, 5), background_color="red", justification="c"),
     sg.T(key="sensor2", font=font, size=(15, 5), background_color="red", justification="c")],
    [sg.T("Then clap. Did all three turn green?", key="hint",
          font=("Arial", 25))],
]

ser = serial.Serial(port="/dev/ttyACM0", baudrate=57600)

window = sg.Window(title="calibration", layout=layout, margins=(200, 200))


def color(val: bool):
    return "green" if val else "red"


while True:
    try:
        (d0, d1, d2), (a0, a1, a2) = map(
            lambda x: map(int, x.split(":")),
            ser.readline().decode().strip().split(";")
        )
    except ValueError as e:
        continue
    window.read(timeout=0)
    window.key_dict["sensor0"].update(value=f"\nD11/A0:\n{a0}", background_color=color(d0))
    window.key_dict["sensor1"].update(value=f"\nD12/A1:\n{a1}", background_color=color(d1))
    window.key_dict["sensor2"].update(value=f"\nD13/A3:\n{a2}", background_color=color(d2))
