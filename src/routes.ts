// Components
import Welcome from './routes/Welcome.svelte'
import SettingsErrorDisplay from './routes/SettingsErrorDisplay.svelte'

// Export the route definition object
export default {
    // Exact path
    '/': Welcome,
    '/settings-load-error': SettingsErrorDisplay,

    // // Using named parameters, with last being optional
    // '/hello/:first/:last?': Name,
    //
    // // Wildcard parameter
    // // Included twice to match both `/wild` (and nothing after) and `/wild/*` (with anything after)
    // '/wild': Wild,
    // '/wild/*': Wild,
    //
    // // Catch-all, must be last
    // '*': NotFound,
}

