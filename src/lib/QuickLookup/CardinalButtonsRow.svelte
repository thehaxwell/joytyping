<script>
  export let buttons = [];
  export let active = false;
</script>
<svg width="148" height="45" viewBox="0 0 148 45" fill="none" xmlns="http://www.w3.org/2000/svg">
  <path d="M16.768 11C17.5378 9.66666 19.4623 9.66667 20.2321 11L32.7894 32.75C33.5592 34.0833 32.597 35.75 31.0574 35.75H5.94263C4.40303 35.75 3.44078 34.0833 4.21058 32.75L16.768 11Z" fill="#D9D9D9" stroke={active ? "blue" : "None"} stroke-width="3"/>
<path d="M71 20.768C72.3333 21.5378 72.3333 23.4623 71 24.2321L49.25 36.7894C47.9167 37.5592 46.25 36.597 46.25 35.0574V9.94263C46.25 8.40303 47.9167 7.44078 49.25 8.21058L71 20.768Z" fill="#D9D9D9" stroke={active ? "blue" : "None"} stroke-width="3"/>
<path d="M94.232 34C93.4622 35.3333 91.5377 35.3333 90.7679 34L78.2106 12.25C77.4408 10.9167 78.403 9.25 79.9426 9.25L105.057 9.25C106.597 9.25 107.559 10.9167 106.789 12.25L94.232 34Z" fill="#D9D9D9" stroke={active ? "blue" : "None"} stroke-width="3"/>
<path d="M114 24.232C112.667 23.4622 112.667 21.5377 114 20.7679L135.75 8.21058C137.083 7.44078 138.75 8.40303 138.75 9.94263V35.0574C138.75 36.597 137.083 37.5592 135.75 36.7894L114 24.232Z" fill="#D9D9D9" stroke={active ? "blue" : "None"} stroke-width="3"/>
<text fill="black" xml:space="preserve" style="white-space: pre" font-family="Inter" font-size="16" letter-spacing="0em"><tspan x="13" y="28.3182">{buttons[0]}</tspan></text>
<text fill="black" xml:space="preserve" style="white-space: pre" font-family="Inter" font-size="16" letter-spacing="0em"><tspan x="50" y="28.3182">{buttons[1]}</tspan></text>
<text fill="black" xml:space="preserve" style="white-space: pre" font-family="Inter" font-size="16" letter-spacing="0em"><tspan x="87" y="28.3182">{buttons[2]}</tspan></text>
<text fill="black" xml:space="preserve" style="white-space: pre" font-family="Inter" font-size="16" letter-spacing="0em"><tspan x="124" y="28.3182">{buttons[3]}</tspan></text>
</svg>
