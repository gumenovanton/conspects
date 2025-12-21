//>> OPTION PATTERN
// применяется когда нужно создавать структуры одного типа с разным набором полей

// например есть структура
type House struct {
    Material     string
    HasFireplace bool
    Floors       int
}

// тип функции
type HouseOption func(*Object)

// для нее я могу определить функции которые устанавливают значения полей
// я могу сделать сколько угодно таких функций
// они могут принимать сколько угодно параметров
// делать какие то операции с ними 
// и возвращают замыкание
func WithConcrete() HouseOption {
    return func(h *House) {
        h.Material = "concrete"
    }
}

func WithoutFireplace() HouseOption {
    return func(h *House) {
        h.HasFireplace = false
    }
}

func WithFloors(floors int) HouseOption {
    return func(h *House) {
        h.Floors = floors
    }
}

// вся магия происходит в конструкторе
func NewHouse(opts ...HouseOption) *House {
    // я создаю объект по умолчанию
    h := &House{
        Material:     "wood",
        HasFireplace: true,
        Floors:       2,
    }

    // и переопределяю только те поля, функции методы для которых были вызваны
    for _, opt := range opts {
        opt(h)
    }
    return h
}

// так я могу создать обьект с любым набором свойств с любой вариацией
h := NewHouse(
    WithConcrete(),
    WithoutFireplace(),
    WithFloors(3),
)