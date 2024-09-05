#!/bin/bash
sgodot=~/Godot/bin/Godot_v3.5.3-stable_linux_server.64
export PROFILE="SERVER"
export EGGZ_PROFILE="1"
$sgodot --path network/Main.tscn
