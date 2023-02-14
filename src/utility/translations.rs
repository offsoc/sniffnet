use crate::enums::language::Language;
use iced::widget::Text;

pub fn choose_adapters_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Select network adapter to inspect",
        Language::IT => "Seleziona la scheda di rete da ispezionare",
        Language::FR => "Sélectionnez une carte réseau à inspecter",
        Language::ES => "Seleccione el adaptador de red que desea inspeccionar",
    })
}

pub fn application_protocol_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Application protocol",
        Language::IT => "Protocollo applicativo",
        Language::FR => "Protocole applicatif",
        Language::ES => "Protocolo de aplicación",
    }
}

pub fn select_filters_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Select filters to be applied on network traffic",
        Language::IT => "Seleziona i filtri da applicare al traffico di rete",
        Language::FR => "Sélectionnez les filtres à appliquer sur le traffic réseau",
        Language::ES => "Seleccionar los filtros que se aplicarán al tráfico de red",
    })
}

pub fn start_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Start!",
        Language::IT => "Avvia!",
        Language::FR => "Commencer!",
        Language::ES => "¡Empieza!",
    }
}

pub fn address_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "\nAddress:",
        Language::IT => "\nIndirizzo:",
        Language::FR => "\nAdresse:",
        Language::ES => "\nDirección:",
    }
}

pub fn addresses_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "\nAddresses:",
        Language::IT => "\nIndirizzi:",
        Language::FR => "\nAdresses:",
        Language::ES => "\nDirecciones:",
    }
}

pub fn ip_version_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "IP version",
        Language::IT => "Versione IP",
        Language::FR => "Version IP",
        Language::ES => "Versión IP",
    })
}

pub fn transport_protocol_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Transport protocol",
        Language::IT => "Protocollo di trasporto",
        Language::FR => "Protocole de transport",
        Language::ES => "Protocolo de transporte",
    }
}

pub fn traffic_rate_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Traffic rate:",
        Language::IT => "Intensità del traffico:",
        Language::FR => "Fréquence du traffic:",
        Language::ES => "Tasa de tráfico:",
    })
}

pub fn relevant_connections_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Relevant connections:",
        Language::IT => "Connessioni rilevanti:",
        Language::FR => "Connexions pertinentes:",
        Language::ES => "Conexiones Relevantes:",
    })
}

pub fn settings_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Settings",
        Language::IT => "Impostazioni",
        Language::FR => "Paramètres",
        Language::ES => "Ajustes",
    }
}

pub fn yes_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Yes",
        Language::IT => "Sì",
        Language::FR => "Oui",
        Language::ES => "Sí",
    })
}

pub fn ask_quit_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Are you sure you want to quit this analysis?",
        Language::IT => "Sei sicuro di voler interrompere questa analisi?",
        Language::FR => "Êtes-vous sûr de vouloir quitter l'application ?",
        Language::ES => "¿Estás seguro de que quieres dejar este análisis?",
    })
}

pub fn quit_analysis_translation(language: Language) -> String {
    match language {
        Language::EN => "Quit analysis".to_string(),
        Language::IT => "Interrompi analisi".to_string(),
        Language::FR => "Quitter l'analyse".to_string(),
        Language::ES => "Quitar el análisis".to_string(),
    }
}

pub fn ask_clear_all_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Are you sure you want to clear notifications?",
        Language::IT => "Sei sicuro di voler eliminare le notifiche?",
        Language::FR => "Êtes-vous sûr de vouloir effacer les notifications ?",
        Language::ES => "¿Seguro que quieres borrar las notificaciones?",
    })
}

pub fn clear_all_translation(language: Language) -> String {
    match language {
        Language::EN => "Clear all".to_string(),
        Language::IT => "Elimina tutte".to_string(),
        Language::FR => "Tout effacer".to_string(),
        Language::ES => "Borrar todo".to_string(),
    }
}

pub fn hide_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Hide",
        Language::IT => "Nascondi",
        Language::FR => "Masquer",
        Language::ES => "Ocultar",
    }
}

pub fn no_addresses_translation(language: Language, adapter: &str) -> Text<'static> {
    Text::new(match language {
        Language::EN => format!("No traffic can be observed because the adapter you selected has no active addresses...\n\n\
                                 Network adapter: {adapter}\n\n\
                                 If you are sure you are connected to the internet, try choosing a different adapter."),
        Language::IT => format!("Non è osservabile alcun traffico perché l'adattatore di rete selezionato non ha indirizzi attivi...\n\n\
                                Adattatore di rete: {adapter}\n\n\
                                Se sei sicuro di essere connesso ad internet, prova a scegliere un adattatore diverso."),
        Language::FR => format!("Aucun trafic ne peut être observé, car la carte réseau que vous avez saisie n'a pas d'adresse...\n\n\
                                Carte réseau : {adapter}\n\n\
                                Si vous êtes sûr d'être connecté à internet, essayez une autre carte."),
        Language::ES => format!("No se puede observar ningún tráfico porque el adaptador seleccionado no tiene direcciones activas...\n\n\
                                 Adaptador de red : {adapter}\n\n\
                                 Si estás seguro de que estás conectado a Internet, prueba a elegir otro adaptador."),
    })
}

pub fn waiting_translation(language: Language, adapter: &str) -> Text<'static> {
    Text::new(match language {
        Language::EN => format!("No traffic has been observed yet. Waiting for network packets...\n\n\
                                 Network adapter: {adapter}\n\n\
                                 Are you sure you are connected to the internet and you have selected the correct adapter?"),
        Language::IT => format!("Nessun tipo di traffico è stato osservato finora. Attendo pacchetti di rete...\n\n\
                                Adattatore di rete: {adapter}\n\n\
                                Sei sicuro di esser connesso ad internet e di aver selezionato l'adattatore corretto?"),
        Language::FR => format!("Aucun trafic n'a été capturé pour le moment. En attente de paquets...\n\n\
                                Carte réseau : {adapter}\n\n\
                                Êtes-vous sûr d'être connecté à internet et d'avoir selectionné la bonne carte réseau ?"),
        Language::ES => format!("Aún no se ha captado tráfico. Esperando paquetes...\n\n\
                                 Adaptador de red : {adapter}\n\n\
                                 ¿Está seguro de que está conectado a Internet y ha seleccionado la tarjeta de red correcta?"),
    })
}

pub fn some_observed_translation(
    language: Language,
    observed: &str,
    filters: &str,
) -> Text<'static> {
    Text::new(match language {
        Language::EN => format!("Total intercepted packets: {observed}\n\n\
                                 Filtered packets: 0\n\n\
                                 Some packets have been intercepted, but still none has been selected according to the filters you specified...\n\n{filters}"),
        Language::IT => format!("Totale pacchetti intercettati: {observed}\n\n\
                                 Pacchetti filtrati: 0\n\n\
                                 Alcuni pacchetti sono stati intercettati, ma ancora nessuno è stato selezionato secondo i filtri specificati...\n\n{filters}"),
        Language::FR => format!("Total des paquets interceptés: {observed}\n\n\
                                 Paquets filtrés: 0\n\n\
                                 Certains paquets ont été interceptés, mais aucun ne satisfait les critères des filtres sélectionnés...\n\n{filters}"),
        Language::ES => format!("Total de paquetes interceptados: {observed}\n\n\
                                 Paquetes filtrados: 0\n\n\
                                 Se interceptaron algunos paquetes, pero ninguno de ellos cumplía los criterios de los filtros seleccionados...\n\n{filters}"),
    })
}

pub fn filtered_packets_translation(
    language: Language,
    filtered: &str,
    percentage: &str,
) -> Text<'static> {
    Text::new(match language {
        Language::EN => format!("Filtered packets:\n   {filtered} ({percentage} of the total)"),
        Language::IT => format!("Pacchetti filtrati:\n   {filtered} ({percentage} del totale)"),
        Language::FR => format!("Paquets filtrés:\n   {filtered} ({percentage} du total)"),
        Language::ES => format!("Paquetes filtrados:\n   {filtered} ({percentage} del total)"),
    })
}

pub fn filtered_bytes_translation(
    language: Language,
    filtered: &str,
    percentage: &str,
) -> Text<'static> {
    Text::new(match language {
        Language::EN => format!("Filtered bytes:\n   {filtered} ({percentage} of the total)"),
        Language::IT => format!("Byte filtrati:\n   {filtered} ({percentage} del totale)"),
        Language::FR => format!("Octets filtrés:\n   {filtered} ({percentage} du total)"),
        Language::ES => format!("Bytes filtrados:\n   {filtered} ({percentage} del total)"),
    })
}

pub fn filtered_application_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Filtered packets per application protocol:",
        Language::IT => "Pacchetti filtrati per protocollo applicativo:",
        Language::FR => "Paquets filtrés par protocole applicatif:",
        Language::ES => "Paquetes filtrados por protocolo de aplicación:",
    })
}

pub fn no_favorites_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Nothing to show at the moment.\n\
                         To add a connection to your favorites, click on the star symbol near the connection.",
        Language::IT => "Nulla da vedere per il momento.\n\
                         Per aggiungere una connessione ai tuoi preferiti, clicca sul simbolo della stella vicino alla connessione.",
        Language::FR => "Rien a voir pour le moment.\n\
                         Pour ajouter une connexion à vos favoris, cliquez sur l'étoile à côté de la connexion.",
        Language::ES => "Nada que mostrar por el momento.\n\
                         Para añadir una conexión a sus favoritos, haga clic en el símbolo de la estrella situado junto a la conexión.",
    })
}

pub fn error_translation(language: Language, error: &str) -> Text<'static> {
    Text::new(match language {
        Language::EN => format!(
            "An error occurred! \n\n\
                                {error}"
        ),
        Language::IT => format!(
            "Si è verificato un errore! \n\n\
                                {error}"
        ),
        Language::FR => format!(
            "Une erreur est survenue! \n\n\
                                {error}"
        ),
        Language::ES => format!(
            "¡Se ha producido un error! \n\n\
                                {error}"
        ),
    })
}

pub fn both_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "both",
        Language::IT => "entrambi",
        Language::FR => "les deux",
        Language::ES => "ambos",
    }
}

// pub fn all_protocols_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "All protocols",
//         Language::IT => "Tutti i protocolli",
//         Language::FR => "Tous les protocoles",
//         Language::ES => "Todos los protocolos",
//     }
// }

pub fn all_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "All",
        Language::IT => "Tutti",
        Language::FR => "Tous",
        Language::ES => "Todos",
    }
}

pub fn packets_chart_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "packets per second",
        Language::IT => "pacchetti al secondo",
        Language::FR => "paquets par seconde",
        Language::ES => "paquetes por segundo",
    }
}

pub fn bytes_chart_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "bytes per second",
        Language::IT => "byte al secondo",
        Language::FR => "octets par seconde",
        Language::ES => "bytes por segundo",
    }
}

pub fn recent_report_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "most recent",
        Language::IT => "più recenti",
        Language::FR => "la plus récente",
        Language::ES => "más reciente",
    }
}

pub fn packets_report_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "most packets",
        Language::IT => "più pacchetti",
        Language::FR => "le plus de paquets",
        Language::ES => "mayoría de los paquetes",
    }
}

pub fn bytes_report_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "most bytes",
        Language::IT => "più byte",
        Language::FR => "le plus de données",
        Language::ES => "mayoría de los bytes",
    }
}

pub fn favorite_report_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "favorites",
        Language::IT => "preferiti",
        Language::FR => "favoris",
        Language::ES => "favoritos",
    }
}

pub fn notifications_title_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Customize your notifications",
        Language::IT => "Personalizza le tue notifiche",
        Language::FR => "Personnalisez vos notifications",
        Language::ES => "Personaliza tus notificaciones",
    })
}

pub fn appearance_title_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Choose your favorite theme",
        Language::IT => "Scegli il tuo tema preferito",
        Language::FR => "Sélectionnez votre thème préféré",
        Language::ES => "Elige tu tema favorito",
    })
}

pub fn languages_title_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Select your language",
        Language::IT => "Seleziona la lingua",
        Language::FR => "Sélectionnez votre langue",
        Language::ES => "Selecciona tu idioma",
    })
}

pub fn active_filters_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Active filters:",
        Language::IT => "Filtri attivi:",
        Language::FR => "Filtres actifs",
        Language::ES => "Filtros activos:",
    }
}

pub fn none_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "none",
        Language::IT => "nessuno",
        Language::FR => "aucun",
        Language::ES => "ninguno",
    }
}

pub fn yeti_night_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Sniffnet's original dark theme",
        Language::IT => "Il tema scuro originale di Sniffnet",
        Language::FR => "Thème original sombre de Sniffnet",
        Language::ES => "Tema oscuro original de Sniffnet",
    }
}

pub fn yeti_day_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Sniffnet's original light theme",
        Language::IT => "Il tema chiaro originale di Sniffnet",
        Language::FR => "Thème original clair de Sniffnet",
        Language::ES => "Tema claro original de Sniffnet",
    }
}

pub fn deep_sea_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "To dive into network traffic",
        Language::IT => "Per immergersi nel traffico di rete",
        Language::FR => "Pour plonger dans votre trafic réseau",
        Language::ES => "Para sumergirse en el tráfico de la red",
    }
}

pub fn mon_amour_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Lovely theme made for dreamers",
        Language::IT => "Tema incantevole fatto per i sognatori",
        Language::FR => "Thème romantique fait pour les rêveurs",
        Language::ES => "Tema encantador hecho para soñadores",
    }
}

pub fn incoming_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Incoming",
        Language::IT => "In entrata",
        Language::FR => "Entrant",
        Language::ES => "Entrante",
    }
}

pub fn outgoing_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Outgoing",
        Language::IT => "In uscita",
        Language::FR => "Sortant",
        Language::ES => "Saliente",
    }
}

pub fn notifications_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::FR => "Notifications",
        Language::IT => "Notifiche",
        Language::ES => "Notificaciones",
    }
}

pub fn style_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::FR => "Style",
        Language::IT => "Stile",
        Language::ES => "Estilo",
    }
}

pub fn language_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Language",
        Language::IT => "Lingua",
        Language::FR => "Langue",
        Language::ES => "Idioma",
    }
}

pub fn overview_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Overview",
        Language::IT => "Panoramica",
        Language::FR => "Résumé",
        Language::ES => "Resumen",
    }
}

// pub fn inspect_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "Inspect",
//         Language::IT => "Ispeziona",
//         Language::FR => "Inspecter",
//         Language::ES => "Inspeccionar",
//     }
// }

pub fn packets_threshold_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Notify me when a packets threshold is exceeded",
        Language::IT => "Notificami quando una soglia di pacchetti è superata",
        Language::FR => "Me notifier lorsqu'un seuil de paquet est atteint",
        Language::ES => "Notificarme cuando se supere un límite de paquetes",
    }
}

pub fn bytes_threshold_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Notify me when a bytes threshold is exceeded",
        Language::IT => "Notificami quando una soglia di byte è superata",
        Language::FR => "Me notifier lorsqu'un seuil de donnée est atteint",
        Language::ES => "Notificarme cuando se exceda un límite de bytes",
    }
}

pub fn per_second_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "(per second)",
        Language::IT => "(al secondo)",
        Language::FR => "(par seconde)",
        Language::ES => "(por segundo)",
    }
}

pub fn specify_multiples_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "; you can also specify 'K', 'M' and 'G'",
        Language::IT => "; puoi anche specificare 'K', 'M' e 'G'",
        Language::FR => "; vous pouvez également spécifier 'K', 'M' et 'G'",
        Language::ES => "; también puede especificar 'K', 'M' y 'G'",
    }
}

pub fn favorite_notification_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Notify me when new data are exchanged from my favorites",
        Language::IT => "Notificami quando nuovi dati sono scambiati dai miei preferiti",
        Language::FR => "Notifiez-moi lorsque des données sont échangées depuis mes favoris",
        Language::ES => "Notificarme cuando se intercambien nuevos datos de mis favoritos",
    }
}

pub fn threshold_translation(language: Language) -> String {
    match language {
        Language::EN => "Threshold: ".to_string(),
        Language::IT => "Soglia: ".to_string(),
        Language::FR => "Seuil: ".to_string(),
        Language::ES => "Límite: ".to_string(),
    }
}

pub fn volume_translation(language: Language, value: u8) -> String {
    match language {
        Language::EN => format!("Volume: {value:^3}%"),
        Language::IT => format!("Volume: {value:^3}%"),
        Language::FR => format!("Volume: {value:^3}%"),
        Language::ES => format!("Volumen: {value:^3}%"),
    }
}

pub fn sound_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Sound:",
        Language::IT => "Suono:",
        Language::FR => "Son:",
        Language::ES => "Sonido:",
    }
}

pub fn open_report_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Open full report",
        Language::IT => "Apri report completo",
        Language::FR => "Ouvrir le rapport complet",
        Language::ES => "Abrir el informe completo",
    }
}

pub fn bytes_exceeded_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Bytes threshold exceeded!",
        Language::IT => "Soglia di Byte superata!",
        Language::FR => "Seuil de donnée atteint!",
        Language::ES => "¡Límite de bytes superado!",
    }
}

pub fn bytes_exceeded_value_translation(language: Language, value: &str) -> String {
    let trimmed_value = value.trim();
    match language {
        Language::EN => format!("{trimmed_value} bytes have been exchanged"),
        Language::IT => format!("{trimmed_value} byte sono stati scambiati"),
        Language::FR => format!("{trimmed_value} octets ont été échangé"),
        Language::ES => format!("{trimmed_value} byte/s han sido intercambiado/s"),
    }
}

pub fn packets_exceeded_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Packets threshold exceeded!",
        Language::IT => "Soglia di pacchetti superata!",
        Language::FR => "Le seuil de paquet a été atteint!",
        Language::ES => "¡Se ha superado el límite de paquetes!",
    }
}

pub fn packets_exceeded_value_translation(language: Language, value: u32) -> String {
    match language {
        Language::EN => format!("{value} packets have been exchanged"),
        Language::IT => format!("{value} pacchetti sono stati scambiati"),
        Language::FR => match value {
            1 => "1 paquet a été échangé".to_owned(),
            npackets => format!("{npackets} paquets ont été échangés"),
        },
        Language::ES => format!("{value} paquete/s han sido intercambiado/s"),
    }
}

pub fn favorite_transmitted_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "New data exchanged from favorites!",
        Language::IT => "Nuovi dati scambiati dai preferiti!",
        Language::FR => "Nouvel échange de donnée depuis un favori!",
        Language::ES => "¡Nuevos datos intercambiados de favoritos!",
    }
}

pub fn no_notifications_set_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "You haven't enabled notifications yet!\n\n\
                                 After you will enable them, this page will display a log of your notifications\n\n\
                                 You can enable notifications from settings:",
        Language::IT => "Non hai ancora abilitato le notifiche!\n\n\
                                Dopo che le avrai abilitate, questa pagina mostrerà una collezione delle tue notifiche\n\n\
                                Puoi abilitare le notifiche dalle impostazioni:",
        Language::FR => "Vous n'avez pas activé les notifications!\n\n\
                                    Une fois activées, cette page affichera le journal des notifications\n\n\
                                    Vous pouvez les activer dans les paramètres:",
        Language::ES => "¡Aún no has activado las notificaciones!\n\n\
                                 Después de activarlas, esta página mostrará un registro de sus notificaciones\n\n\
                                 Puedes activar las notificaciones desde los ajustes:",
    })
}

pub fn no_notifications_received_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => {
            "Nothing to see at the moment...\n\n\
                                 When you will receive a notification, it will be displayed here"
        }
        Language::IT => {
            "Nulla da vedere al momento...\n\n\
                                Quando riceverai una notifica, essa verrà mostrata qui"
        }
        Language::FR => {
            "Rien à voir pour le moment...\n\n\
                                 Lorsque vous recevrez une notification, elle s'affichera ici"
        }
        Language::ES => {
            "Nada que ver por el momento...\n\n\
                                 Cuando reciba una notificación, aparecerá aquí"
        }
    })
}

pub fn only_last_30_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Only the last 30 notifications are displayed",
        Language::IT => "Solo le ultime 30 notifiche sono mostrate",
        Language::FR => "Seulement les 30 dernières notifications sont affichées",
        Language::ES => "Sólo se muestran las últimas 30 notificaciones",
    }
}
