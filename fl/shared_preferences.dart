// value key store

// create a storage
final sharedPrefs = await ShoredPreferences.getInstance();

// add data
sharedPrefs.setString('some_key', 'value');
sharedPrefs.setBool('boolean', true);
sharedPrefs.setInt('integer', 3);
sharedPrefs.setDouble('double', 2.43);

// get data
final str = sharedPrefs.getString('some_key');
final boolean = sharedPrefs.getBool('boolean');
final int = sharedPrefs.getInt('integer');
final double = sharedPrefs.getDouble('double');

// clear storage
sharedPrefs.clear();

// remove an item
sharedPrefs.remove('some_key')

// is contains an item
sharedPrefs.containsKey('some_key')
