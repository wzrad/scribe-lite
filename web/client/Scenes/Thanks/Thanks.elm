module Scenes.Thanks.Thanks exposing (view, background)

import Html exposing (..)
import Css exposing (Color)
import Scenes.Thanks.Styles exposing (Classes(..), styles)
import Views.Button as Button
import Styles.Colors as Colors

-- constants
background : Color
background =
  Colors.primary

-- view
{ class, classes } = styles

view : a -> Html m
view model =
  div [ class Scene ]
    [ p [ class Message ]
      [ text "Thanks for writing" ]
    , p [ class Message ]
      [ text "At 8PM tonight, today's story will be e-mailed to you." ]
    , div [ class Button ]
      [ Button.view "Refresh Page" True ]
    ]
