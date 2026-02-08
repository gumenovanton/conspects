//<< Unmarshal() 
// Unmarshal from bytes to type 
type Config struct {
	Database struct {
		Host     string `yaml:"host"`
		Port     int    `yaml:"port"`
		Username string `yaml:"username"`
		Password string `yaml:"password"`
	} `yaml:"database"`
}

data, _ := os.ReadFile("config.yaml")

var config Config
err = yaml.Unmarshal(data, &config)

//<< Marshal() 
// Marshal tyoe to bytes 
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
bytes, _ := yaml.Marshal(&config)