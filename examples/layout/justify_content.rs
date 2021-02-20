use tuix::*;


fn main() {

    let app = Application::new(|win_desc, state, window| {


        let window_flex_direction = FlexDirection::Row;

        window.set_background_color(state, Color::white()).set_flex_direction(state, window_flex_direction);


        let container_flex_direction = match window_flex_direction {
            FlexDirection::Column => {
                FlexDirection::Row
            }

            FlexDirection::Row => {
                FlexDirection::Column
            }
        };

        //////////////////
        /// Flex Start ///
        //////////////////
        let container = Element::new().build(state, window, |builder| 
            builder
                .set_flex_grow(1.0)
                .set_background_color(Color::rgb(50,50,50))
                .set_justify_content(JustifyContent::FlexStart)
                .set_align_items(AlignItems::Center)
                .set_flex_direction(container_flex_direction)
        );

        Element::new().build(state, container, |builder| 
            builder
                .set_width(Length::Pixels(30.0))
                .set_height(Length::Pixels(30.0))
                .set_background_color(Color::rgb(50,50,120))
        );

        Element::new().build(state, container, |builder| 
            builder
                .set_width(Length::Pixels(30.0))
                .set_height(Length::Pixels(30.0))
                .set_background_color(Color::rgb(50,50,160))
        );

        Element::new().build(state, container, |builder| 
            builder
                .set_width(Length::Pixels(30.0))
                .set_height(Length::Pixels(30.0))
                .set_background_color(Color::rgb(50,50,200))
        );

        //////////////////
        /// Flex End   ///
        //////////////////
        let container = Element::new().build(state, window, |builder| 
            builder
                .set_flex_grow(1.0)
                .set_background_color(Color::rgb(70,70,70))
                .set_justify_content(JustifyContent::FlexEnd)
                .set_align_items(AlignItems::Center)
                .set_flex_direction(container_flex_direction)
        );


        Element::new().build(state, container, |builder| 
            builder
                .set_width(Length::Pixels(30.0))
                .set_height(Length::Pixels(30.0))
                .set_background_color(Color::rgb(50,120,50))
        );

        Element::new().build(state, container, |builder| 
            builder
                .set_width(Length::Pixels(30.0))
                .set_height(Length::Pixels(30.0))
                .set_background_color(Color::rgb(50,160,50))
        );

        Element::new().build(state, container, |builder| 
            builder
                .set_width(Length::Pixels(30.0))
                .set_height(Length::Pixels(30.0))
                .set_background_color(Color::rgb(50,200,50))
        );

        //////////////////
        /// Center     ///
        //////////////////
        let container = Element::new().build(state, window, |builder| 
            builder
                .set_flex_grow(1.0)
                .set_background_color(Color::rgb(50,50,50))
                .set_justify_content(JustifyContent::Center)
                .set_align_items(AlignItems::Center)
                .set_flex_direction(container_flex_direction)
        );


        Element::new().build(state, container, |builder| 
            builder
                .set_width(Length::Pixels(30.0))
                .set_height(Length::Pixels(30.0))
                .set_background_color(Color::rgb(120,50,50))
        );

        Element::new().build(state, container, |builder| 
            builder
                .set_width(Length::Pixels(30.0))
                .set_height(Length::Pixels(30.0))
                .set_background_color(Color::rgb(160,50,50))
        );

        Element::new().build(state, container, |builder| 
            builder
                .set_width(Length::Pixels(30.0))
                .set_height(Length::Pixels(30.0))
                .set_background_color(Color::rgb(200,50,50))
        );

        ////////////////////
        /// Space Around ///
        ////////////////////
        let container = Element::new().build(state, window, |builder| 
            builder
                .set_flex_grow(1.0)
                .set_background_color(Color::rgb(70,70,70))
                .set_justify_content(JustifyContent::SpaceAround)
                .set_align_items(AlignItems::Center)
                .set_flex_direction(container_flex_direction)
        );


        Element::new().build(state, container, |builder| 
            builder
                .set_width(Length::Pixels(30.0))
                .set_height(Length::Pixels(30.0))
                .set_background_color(Color::rgb(120,120,50))
        );

        Element::new().build(state, container, |builder| 
            builder
                .set_width(Length::Pixels(30.0))
                .set_height(Length::Pixels(30.0))
                .set_background_color(Color::rgb(160,120,50))
        );

        Element::new().build(state, container, |builder| 
            builder
                .set_width(Length::Pixels(30.0))
                .set_height(Length::Pixels(30.0))
                .set_background_color(Color::rgb(200,120,50))
        );


        /////////////////////
        /// Space Between ///
        /////////////////////
        let container = Element::new().build(state, window, |builder| 
            builder
                .set_flex_grow(1.0)
                .set_background_color(Color::rgb(50,50,50))
                .set_justify_content(JustifyContent::SpaceBetween)
                .set_align_items(AlignItems::Center)
                .set_flex_direction(container_flex_direction)
        );

        Element::new().build(state, container, |builder| 
            builder
                .set_width(Length::Pixels(30.0))
                .set_height(Length::Pixels(30.0))
                .set_background_color(Color::rgb(120,50,120))
        );

        Element::new().build(state, container, |builder| 
            builder
                .set_width(Length::Pixels(30.0))
                .set_height(Length::Pixels(30.0))
                .set_background_color(Color::rgb(160,50,160))
        );

        Element::new().build(state, container, |builder| 
            builder
                .set_width(Length::Pixels(30.0))
                .set_height(Length::Pixels(30.0))
                .set_background_color(Color::rgb(200,50,200))
        );

        /////////////////////
        /// Space Evenly  ///
        /////////////////////
        let container = Element::new().build(state, window, |builder| 
            builder
                .set_flex_grow(1.0)
                .set_background_color(Color::rgb(70,70,70))
                .set_justify_content(JustifyContent::SpaceEvenly)
                .set_align_items(AlignItems::Center)
                .set_flex_direction(container_flex_direction)
        );

        Element::new().build(state, container, |builder| 
            builder
                .set_width(Length::Pixels(30.0))
                .set_height(Length::Pixels(30.0))
                .set_background_color(Color::rgb(50,120,120))
        );

        Element::new().build(state, container, |builder| 
            builder
                .set_width(Length::Pixels(30.0))
                .set_height(Length::Pixels(30.0))
                .set_background_color(Color::rgb(50,160,160))
        );

        Element::new().build(state, container, |builder| 
            builder
                .set_width(Length::Pixels(30.0))
                .set_height(Length::Pixels(30.0))
                .set_background_color(Color::rgb(50,200,200))
        );


        win_desc.with_title("position")
    });

    app.run();
}
