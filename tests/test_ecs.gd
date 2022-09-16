extends GutTest


func test_ecs():
	var gdn = GDNative.new()
	var status = false;
	gdn.library = load("res://tests/test_gnd.gdnlib")
	if gdn.initialize():
		status = gdn.call_native("standard_varcall", "run_tests", [])
		gdn.terminate()
	assert_eq(status, true)
