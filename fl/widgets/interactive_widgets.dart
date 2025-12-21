// showModalBothomSheet() - method to open bottom tab, with the child

// Drawer
// Drawer - opening menu of appbar or of sliver appbar of scaffold
// to define it i need to put a Drawer widget into drower(from left) or endDrawer(from right) property of scaffold
// if scaffold has appbar, drawer will be automatically add to the appbar
// drawer will automatically opened when i click menu icon in appbar
// to handle opening by button click i need to define scaffold global key
// final GlobalKey<ScaffoldState> scaffoldKey = GlobalKey<ScaffoldState>();
// then add this key to key property of scaffold
// and in button onTap run scaffoldKey.currentState!.openEndDrawer();

// DatePicker
// wihget to select a date
// to show it i need to run showDatePicker
// void _selectDate(BuildContext context) async {
//    final picked = await showDatePicker(
//      context: context,
//      initialDate: selectedDate ?? DateTime.now(),
//      // max and min date
//      firstDate: _firstDate,
//      lastDate: _lastDate,
//    );
//
//    if (picked != null && picked != selectedDate) {
//      setState(() {
//        selectedDate = picked;
//      });
//    }
//  }

// TimePicker
// widget to select time
// to show it i need to run showTimePicker func
// void _selectTime(BuildContext context) async {
//     final picked = await showTimePicker(
//       context: context,
//       initialEntryMode: TimePickerEntryMode.input,
//       initialTime: selectedTime ?? TimeOfDay.now(),
//       builder: (context, child) {
//         return MediaQuery(
//           data: MediaQuery.of(context).copyWith(
//             alwaysUse24HourFormat: true,
//           ),
//           child: child!,
//         );
//       },
//     );
//
//     if (picked != null && picked != selectedTime) {
//       setState(() {
//         selectedTime = picked;
//       });
//     }
//   }
