# УСТАНОВКА ИНЛАЙН СТИЛЕЙ

// это можно сделать так, что эквивалент
<div style:color="red">...</div>
<div style="color: red;">...</div>

// или так , если это переменная, что эквивалент
<div style:color={myColor}>...</div>
<div style:color>...</div>

// так можно поставить несколько
<div style:color style:width="12rem" style:background-color={darkMode ? 'black' : 'white'}>...</div>

// так можно поставить important
<div style:color|important="red">...</div>

// и можно миксовать
<div style="color: blue;" style:color="red">This will be red</div>
