//>> Encode 
// все так же как и в json 
// нужно создать энкодер и закодировать в Writer 
config := Config{
    Database: struct {
        Host     string `yaml:"host"`
        Port     int    `yaml:"port"`
        Username string `yaml:"username"`
        Password string `yaml:"password"`
    }{
        Host:     "localhost",
        Port:     5432,
        Username: "admin",
        Password: "password123",
    },
}

file, err := os.Create("config.yaml")  
encoder := yaml.NewEncoder(file)
encoder.Encode(config)

//>> Decode 
config := Config{
    Database: struct {
        Host     string `yaml:"host"`
        Port     int    `yaml:"port"`
        Username string `yaml:"username"`
        Password string `yaml:"password"`
    }{},
}

file, err := os.Create("config.yaml")  
decoder := yaml.NewDecoder(file)
decoder.Decode(&config)