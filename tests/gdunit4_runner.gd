extends SceneTree

## Guard runner for repository-level Godot test execution.
##
## This script exists so CI has a stable command shape before the approved
## GUT/GdUnit4 addon is installed and wired. It must fail loudly rather than
## reporting a false "0 tests passed" success.

const GUT_PLUGIN_PATH := "res://addons/gut/plugin.cfg"
const GDUNIT4_PLUGIN_PATH := "res://addons/gdUnit4/plugin.cfg"


func _init() -> void:
	var has_gut := FileAccess.file_exists(GUT_PLUGIN_PATH)
	var has_gdunit4 := FileAccess.file_exists(GDUNIT4_PLUGIN_PATH)

	if not has_gut and not has_gdunit4:
		_print_missing_framework_error()
		quit(1)
		return

	_print_delegation_missing_error(has_gut, has_gdunit4)
	quit(1)


func _print_missing_framework_error() -> void:
	printerr("ERROR: No approved Godot GDScript test framework is installed.")
	printerr("Checked paths:")
	printerr("- %s" % GUT_PLUGIN_PATH)
	printerr("- %s" % GDUNIT4_PLUGIN_PATH)
	printerr("")
	printerr("Install the approved GUT/GdUnit4 addon and replace this guard with")
	printerr("a real runner delegation before enabling passing CI test evidence.")
	printerr("This command exits with failure so CI cannot falsely pass.")


func _print_delegation_missing_error(has_gut: bool, has_gdunit4: bool) -> void:
	printerr("ERROR: A Godot test addon was detected, but this guard runner has")
	printerr("not yet been replaced with a real framework delegation.")
	printerr("Detected GUT: %s" % str(has_gut))
	printerr("Detected GdUnit4: %s" % str(has_gdunit4))
	printerr("")
	printerr("Wire the selected runner explicitly and keep CI failing until real")
	printerr("tests execute and report actual pass/fail results.")
