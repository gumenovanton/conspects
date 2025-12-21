# INHERITED NOTIFIER

# HOW TO USE

// STEP1: create a WiewModel layer, here must be a bisness logic, extends ChangeNotivier to auto notify listeners to rebuild with notifyListeners()
// create a state into it
// create changing state methods, and notify listeners into it
class CalcModel extends ChangeNotifier {
  int? _valueOne;
  int? _valueTwo;
  int? result;

  set firstNumber(String value) => _valueOne = int.tryParse(value);
  set secondNumber(String value) => _valueTwo = int.tryParse(value);

  void sum() {
    if (_valueOne != null && _valueTwo != null) {
      result = _valueOne! + _valueTwo!;
    } else {
      result = null;
    }

    // notify all listeners to rebuild
    notifyListeners();
  }
}

// STEP2: create a data provider class, extends of an InheritedNotifier<T> when it rebuilds, all dependent widgets will be rebuilt
// create two methods for widgets to subscribe and for widgets to just read the state
class CalcWidgetProvider extends InheritedNotifier<CalcModel> {
  final CalcModel model;

  const CalcWidgetProvider({
    super.key,
    required this.model,
    required super.child,
  }) : super(notifier: model);

  // ATTANTION: of method to get model quickly and subscribe on it
  static CalcModel? watch(BuildContext context) {
    return context
        .dependOnInheritedWidgetOfExactType<CalcWidgetProvider>()
        ?.model;
  }

  // ATTANTION: of method to get model quickly but don't subscribe on it, i need it because i do not want to rebuiri textfields and buttons
  static CalcModel? read(BuildContext context) {
    final widget = context.getElementForInheritedWidgetOfExactType<CalcWidgetProvider>()?.widget;
    return widget is CalcWidgetProvider ? widget.notifier:null;
  }
}

// STEP3: create a widget that stores viewModel, stateful only for storing VM layer, newer will be rebuilt because no setState invoking here
// inject the model
// wrap dependent widgets with provider
class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key});

  @override
  State<MyHomePage> createState() {
    return _MyHomePageState();
  }
}

class _MyHomePageState extends State<MyHomePage> {
  // ATTANTION: create VM injection
  final _model = CalcModel();

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: EdgeInsets.all(30),
      // ATTANTION: here i should wrap my dependent widgets into provider class, and pass VM entity
      child: CalcWidgetProvider(
        model: _model,
        child: const Column(
          children: [
            AppTextField1(),
            SizedBox(height: 10),
            AppTextField2(),
            SizedBox(height: 10),
            CalcButton(),
            SizedBox(height: 10),
            Result(),
          ],
        ),
      ),
    );
  }
}

// STEP4: create consumers widgets as stateless
// widgets that does not need to rebuild only read on inherited provider
// widgets than needs to rebuild subscribe on inherited provider changes
class AppTextField1 extends StatelessWidget {
  const AppTextField1({super.key});

  @override
  Widget build(BuildContext context) {
    return TextField(
      // ATTANTION: here just execute a setter
      onChanged: (value) => CalcWidgetProvider.read(context)?.firstNumber = value,
    );
  }
}

class AppTextField2 extends StatelessWidget {
  const AppTextField2({super.key});

  @override
  Widget build(BuildContext context) {
    return TextField(
      // ATTANTION: here just execute a setter
      onChanged: (value) =>
          CalcWidgetProvider.read(context)?.secondNumber = value,
    );
  }
}

class CalcButton extends StatelessWidget {
  const CalcButton({super.key});

  @override
  Widget build(BuildContext context) {
    return TextButton(
      // ATTANTION: here just execute a businesLogic
      onPressed: () {
        CalcWidgetProvider.read(context)?.sum();
      },
      child: const Text('Calculate'),
    );
  }
}

class Result extends StatelessWidget {
  const Result({super.key});

  @override
  Widget build(BuildContext context) {
    final result = CalcWidgetProvider.watch(context)?.result ?? 0;
    return Text('$result');
  }
}
