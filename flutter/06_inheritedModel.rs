# INHERITED MODEL
- widget to provide multy state to its children
- and defferent widgets can depend on different state

# HOW TO WORK WITH IT
// STEP #1 - create a class that extends from IngeritedModel
// i must defind aspect type, here is the String type
class DataProviderInherited extends InheritedModel<String>{
    // stata to share
    final int valueOne;
    final int valueTwo;

    DataProviderInherited({
       Key?:key,
       required this.valueOne,
       required this.valueTwo,
       required child
    }): super(key:key, child:child);

    // method for check if shared state is changed, then update subscribed widgets
    @override
    bool updateShouldNotify(DataProviderInherited oldWidget){
        return valueOne != oldWidget.valueOne || valueTwo != oldWidget.valueTwo;
    }

    // and i must define this method, that deside whitch widget must be rebuilt
    @override
    bool updateShouldNotifyDepependent(covariant DataProviderInherited oldWidget, Set<String> aspect){
        return (valueOne!=oldWidget.valueOne && aspect.contains('one')) && (valueTwo!=oldWidget.valueTwo && aspect.contains('two'));
    }
}

// STEP #2 - in widget that changes state(here valueOne and valueTwo), fe page
// wrap child widgets that has to be dependent on this state, in DataProviderInherited
// and pass state(valueOne & valueTwo) in valueOne & valueTwo params of DataProviderInherited
class HomePage extends StatefulWidget{
    const HomePage ({super.key});

    @override
    State<HomePage> createState()=> _HomePageState();
}

class _HomePageState extends State<HomePage>{
    var _value1 = 0;
    var _value2 = 0;

    void _increment1(){
        _value1+=1;
        setState((){});
    }

    void _increment2(){
        _value2+=1;
        setState((){});
    }

    @override
    Widget build(BuildContext context){
        return Column(
            children:[
                TextButton('+', onPressed: _increment1()),
                TextButton('+', onPressed: _increment2()),
                // when _value will change, this widget will be rebuilt, and will be created new DataProviderInherited
                // then will be executed updateShouldNotified() where state of old DataProviderInherited
                // will be compared with state of new DataProviderInherited
                // and all subscribed widgets will be rebuilt
                DataProviderInherited(
                    valueOne: _value1;
                    valueTwo: _value2;
                    child: Column(
                        children: [
                            SomeConsumerWIdget1(),
                            SomeConsumerWIdget2(),
                        ],
                    )
                )
            ],
        );
    }
}

// STEP #3 - go to dependent widget, that should be updated when state will be changed
// that can be a slw
class SomeConsumerWidget1 extends StatelessWidget{
    const SomeConsumerWidget1({Key? key}):super(key:key);

    @override
    Widget build(BuildContext context){
        // here a get valueOne
        // method build of this vidget will be executed when valueOne will be changed
        // because i use InheritedModel i must pass an aspect, to check on whitch state the widget should rebuild
        final value = context.dependOnInheritedWidgetOfExactType<DataProviderInherited>(aspect: 'one')?.valueOne ?? 0;

        return Text('$value');
    }
}

class SomeConsumerWidget2 extends StatelessWidget{
    const SomeConsumerWidget2({Key? key}):super(key:key);

    @override
    Widget build(BuildContext context){
        // here a get valueTwo
        // method build of this vidget will be executed when valueTwo will be changed
        // because i use InheritedModel i must pass an aspect, to check on whitch state the widget should rebuild
        final value = context.dependOnInheritedWidgetOfExactType<DataProviderInherited>(aspect: 'two')?.valueTwo ?? 0;

        return Text('$value');
    }
}
