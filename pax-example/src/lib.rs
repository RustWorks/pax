use pax::*;
use pax_std::{Stacker, Text, Rectangle, StackerDirection};

#[pax_root(
    //Fill screen with ten even columns
    <Stacker cell_count=10 >

        //First column: split into five rows
        <Stacker cell_count=5 direction=StackerDirection::Vertical >
            for i in 0..5 {
                <Rectangle fill={
                    rgb((i * 20)%, 0, 100%)
                }/>
            }
        </Stacker>

        //Middle eight columns
        for i in 0..8 {
            <Group>
                <Text id=index_text>{i}</Text>
                <Rectangle fill={
                    rgb(100%, (100 - (i * 12.5))%,(i * 12.5)%)
                }/>
            </Group>
        }

        //Final column: clickable, animated
        <Group @click=self.handle_click transform={rotate(self.current_rotation)}>
            <Text>{JABBERWOCKY}</Text>
            <Rectangle fill=rgb(100%, 100%, 0) />
        </Group>

    </Stacker>

    @settings {
        #index_text {
            transform: { align(0%, (i * 12.5)%) }
            font: {
                family: "Real Text Pro",
                variant: "Demibold",
                size: {( 20 + (i * 5))px},
            }
            fill: rgb(0,0,0)
        }
    }
)]
pub struct Jabberwocky {
    pub num_clicks : Property<i64>,
    pub current_rotation: Property<f64>,
}

impl Jabberwocky {

    #[pax_on(WillRender)] 
    pub fn handle_will_render(&mut self, args: ArgsTick) {
        if args.frames_elapsed % 180 == 0 {
            //every 3s
            pax::log(&format!("pax::log from frame {}", args.frames_elapsed));
        }
    }

    pub fn handle_click(&mut self, args: ArgsClick) {
        let new_rotation = self.current_rotation.get() + (2.0 * std::f64::consts::PI);
        self.current_rotation.ease_to(new_rotation, 120, EasingCurve::InOutBack );
        self.current_rotation.ease_to_later(0.0, 1, EasingCurve::Linear );
    }
}

const JABBERWOCKY : &str = r#"’Twas brillig, and the slithy toves
Did gyre and gimble in the wabe:
All mimsy were the borogoves,
And the mome raths outgrabe.

“Beware the Jabberwock, my son!
The jaws that bite, the claws that catch!
Beware the Jubjub bird, and shun
The frumious Bandersnatch!”

He took his vorpal sword in hand;
Long time the manxome foe he sought—
So rested he by the Tumtum tree
And stood awhile in thought.

And, as in uffish thought he stood,
The Jabberwock, with eyes of flame,
Came whiffling through the tulgey wood,
And burbled as it came!

One, two! One, two! And through and through
The vorpal blade went snicker-snack!
He left it dead, and with its head
He went galumphing back.

“And hast thou slain the Jabberwock?
Come to my arms, my beamish boy!
O frabjous day! Callooh! Callay!”
He chortled in his joy.

’Twas brillig, and the slithy toves
Did gyre and gimble in the wabe:
All mimsy were the borogoves,
And the mome raths outgrabe.
"#;

